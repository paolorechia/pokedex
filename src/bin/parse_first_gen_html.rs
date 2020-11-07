use pokedex_scraper::mongo::init_pokemon_collection;
use pokedex_scraper::{update_pokemon_to_mongo, find_pokemon_by_name};
use select::document::Document;
use select::predicate::{Class, Name};
use std::boxed::Box;
use std::result::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = Document::from(include_str!("../../data/first_gen.html"));

    let wikitable = document.find(Class("wikitable")).next().unwrap();

    let descendants = wikitable.children();
    let collection = init_pokemon_collection().await?;
    for d in descendants {
        let name = d.name();
        if name.is_some() && name.unwrap() == "tbody" {
            let rows = d.children();
            for row in rows {
                let is_row = row.name().is_some() && row.name().unwrap() == "tr";
                if is_row {
                    let th = row.find(Name("th")).next().unwrap();
                    let mut name = String::from(th.text().replace("\n", ""));
                    name = name.trim().to_string();
                    let maybe_pokemon = find_pokemon_by_name(&collection, &name).await?;
                    match maybe_pokemon {
                        Some(mut poke) => {
                            println!("Updating {:?}", poke);
                            poke.generation = 1;
                            update_pokemon_to_mongo(&collection, poke).await?;
                            let maybe_pokemon = find_pokemon_by_name(&collection, &name).await?;
                            if maybe_pokemon.is_some() {
                                println!("{:?}", maybe_pokemon.unwrap());
                            }
                        },
                        None => ()
                    }
                }
            }
        }
    }

    Ok(())
}
