use std::boxed::Box;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::result::Result;
use std::vec::Vec;

use scraper::{Html, Selector};

use pokedex_scraper::load_config;

use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = load_config();

    let data_folder = Path::new(&settings.data_folder);
    let pokemon_list_html = data_folder.join(&settings.root_html_file);

    let html = fs::read_to_string(pokemon_list_html).expect("Failed to open HTML file");
    let doc = Html::parse_document(&html);

    let selector = Selector::parse("a").unwrap();
    let pokemons_set: HashSet<&str> = doc
        .select(&selector)
        .map(|x| {
            let href = x.value().attr("href");
            let title = x.value().attr("title");
            return (href, title);
        })
        .filter(|x| match x {
            (Some(_), Some(_)) => true,
            (_, _) => false,
        })
        .map(|x| (x.0.unwrap(), x.1.unwrap()))
        .filter(|t| t.0.contains("/pt-br/wiki/") && t.0.contains(t.1))
        .map(|t| t.1)
        .collect();

    let mut pokemons: Vec<&str> = pokemons_set.into_iter().collect();
    pokemons.sort();
    let serialized = serde_json::to_string(&pokemons).unwrap();

    let out = data_folder.join(&settings.poke_list_file);
    println!("Saving to... {}", out.display());

    fs::write(out, serialized).expect("Could not write to file!!");

    println!("Finished! {} pokemons found.", pokemons.len());
    Ok(())
}
