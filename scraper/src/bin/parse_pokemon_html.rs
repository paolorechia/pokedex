use pokedex_scraper::config;
use pokedex_scraper::mongo::init_pokemon_collection;
use pokedex_scraper::{load_config, load_pokemon_html, load_pokemon_list, save_pokemon_to_mongo};
use model::Pokemon;
use scraper::{ElementRef, Html, Selector};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::thread;
use std::vec::Vec;
use tokio::runtime::Runtime;

#[derive(Debug)]
struct Blocks {
    origin: String,
    name_origin: String,
    evolution: String,
}

#[derive(Debug)]
struct SmallTable {
    category: String,
    height: String,
    weight: String,
    types: String,
}

fn empty_table() -> SmallTable {
    SmallTable {
        category: "".to_string(),
        height: "".to_string(),
        weight: "".to_string(),
        types: "".to_string(),
    }
}

fn strip_tags(html: &str) -> String {
    let frag = scraper::Html::parse_fragment(html);
    let mut stripped: Vec<String> = vec![];
    for node in frag.tree {
        if let scraper::node::Node::Text(text) = node {
            stripped.push(text.text.to_string());
        }
    }
    stripped.join("")
}

fn find_description(doc: &Html) -> String {
    let selector = Selector::parse("div.mw-parser-output").unwrap();
    let x = doc.select(&selector).next();

    match x {
        Some(x) => {
            /* Picks first two paragraphs */
            let p_sel = &Selector::parse("p").unwrap();
            let mut paragraphs = x.select(p_sel);

            let p = paragraphs.next();

            match p {
                Some(p) => {
                    let stripped_p = strip_tags(&p.inner_html());
                    let p2 = paragraphs.next();
                    match p2 {
                        Some(p2) => {
                            /* Strips tags and concatenates */
                            let stripped_p2 = strip_tags(&p2.inner_html());
                            (stripped_p + &stripped_p2).to_string()
                        }
                        None => stripped_p,
                    }
                }
                None => "".to_string(),
            }
        }
        None => "".to_string(),
    }
}

fn find_blocks(doc: &Html) -> Blocks {
    let selector = Selector::parse("div.mw-parser-output").unwrap();
    let div = doc.select(&selector).next().unwrap();

    let mut blocks: HashMap<String, Vec<String>> = HashMap::new();

    let mut current_headline: String = "".to_string();
    for child in div.children() {
        let elem = ElementRef::wrap(child);
        match elem {
            Some(el) => {
                let span_selector = Selector::parse("h3 > .mw-headline").unwrap();
                let span = el.select(&span_selector).next();
                match span {
                    Some(s) => {
                        current_headline = strip_tags(&s.inner_html());
                        blocks.insert(current_headline.clone(), vec![]);
                    }
                    _ => {
                        if !current_headline.is_empty() {
                            let s = strip_tags(&el.inner_html());
                            let v = blocks.get_mut(&current_headline);
                            match v {
                                Some(v) => v.push(s),
                                None => (),
                            };
                        }
                    }
                }
            }
            None => (),
        };
    }
    let o = blocks.get("Origem");
    let origin = match o {
        Some(o) => o.join(""),
        None => "".to_string(),
    };
    let no = blocks.get("Origem do nome");
    let name_origin = match no {
        Some(no) => no[0].clone(),
        None => "".to_string(),
    };
    let ev = blocks.get("Evolução");
    let evolution = match ev {
        Some(ev) => ev[0].clone(),
        None => "".to_string(),
    };

    Blocks {
        origin,
        name_origin,
        evolution,
    }
}

fn find_table(doc: &Html) -> SmallTable {
    let selector = Selector::parse("div.mw-parser-output > table").unwrap();
    let mut tables = doc.select(&selector);

    tables.next();
    let second = tables.next();
    match second {
        Some(table) => {
            let sel = Selector::parse("tr").unwrap();

            let desired_fields: HashSet<&'static str> = ["Categoria", "Tipo(s)", "Altura", "Peso"]
                .iter()
                .cloned()
                .collect();
            let mut fields: HashMap<String, String> = HashMap::new();
            let rows = table.select(&sel);

            let title_selector = Selector::parse("td > b").unwrap();
            for row in rows {
                let title_cell = row.select(&title_selector).next();
                let title = match title_cell {
                    Some(title_cell) => {
                        let title = strip_tags(&title_cell.inner_html());
                        title
                    }
                    _ => "".to_string(),
                };
                if !title.is_empty() && desired_fields.contains(title.as_str()) {
                    let td_selector = Selector::parse("td").unwrap();
                    let mut tds = row.select(&td_selector);

                    // Skip first
                    tds.next();

                    let value_cell = tds.next();
                    match value_cell {
                        Some(value_cell) => {
                            let value = strip_tags(&value_cell.inner_html());
                            fields.insert(title, value);
                        }
                        None => {}
                    }
                }
            }
            SmallTable {
                category: fields
                    .get("Categoria")
                    .unwrap_or(&"".to_string())
                    .to_string(),
                height: fields.get("Altura").unwrap_or(&"".to_string()).to_string(),
                weight: fields.get("Peso").unwrap_or(&"".to_string()).to_string(),
                types: fields.get("Tipo(s)").unwrap_or(&"".to_string()).to_string(),
            }
        }
        _ => empty_table(),
    }
}

fn raw_data_to_pokemon(name: String, d: String, b: Blocks, t: SmallTable) -> Pokemon {
    Pokemon {
        id: None,
        name: name,
        description: d,
        origin: b.origin,
        name_origin: b.name_origin,
        evolution: b.evolution,
        category: t.category,
        height: t.height,
        weight: t.weight,
        pokemon_types: vec![t.types],
        generation: -1,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = load_config();

    let pokemons = load_pokemon_list(&settings);
    let mut handles = vec![];

    // let y: String = "Bulbasaur".to_string();
    // pokemons.retain(|x| *x == y);

    // Split into chunks
    let threads = 20;
    let mut i = 0;
    let mut j = 0;
    let chunk_size = pokemons.len() / threads;
    let mut chunk_number = pokemons.len() / chunk_size;
    if pokemons.len() % chunk_size > 0 {
        chunk_number += 1;
    }

    let mut chunks = vec![];
    for _ in 0..chunk_number {
        chunks.push(vec![])
    }
    for pokemon in pokemons {
        if i == chunk_size {
            j += 1;
            i = 0;
        }
        chunks[j].push(pokemon);
        i += 1;
    }

    println!("Splitting it into chunks!");
    let shared_settings = Arc::new(settings);
    for i in 0..threads {
        let collection = init_pokemon_collection().await?;
        println!("Thread: {}", i);
        let chunk = chunks[i].clone();
        println!("Chunk: {:?}", chunk);
        let settings = shared_settings.clone();
        let handle = thread::spawn(move || {
            let mut rt = Runtime::new().unwrap();
            rt.block_on(async_processing(chunk, &settings, &collection))
                .expect("Error spawning async task");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

async fn async_processing(
    chunk: Vec<String>,
    settings: &Arc<config::Settings>,
    collection: &mongodb::Collection,
) -> Result<(), Box<dyn std::error::Error>> {
    for chosen in chunk {
        let html = load_pokemon_html(&settings, &chosen);
        match html {
            Some(html) => {
                let doc = Html::parse_document(&html);
                let d = find_description(&doc);
                let b = find_blocks(&doc);
                let t = find_table(&doc);
                println!("Description: {}", d);
                println!("Blocks : {:?}", b);
                println!("Table: {:?}", t);
                println!("Saving to mongo...");
                let pokemon = raw_data_to_pokemon(chosen.clone(), d, b, t);
                save_pokemon_to_mongo(&collection, pokemon).await?;
            }
            None => {
                println!("File for not found!");
            }
        };
    }
    Ok(())
}
