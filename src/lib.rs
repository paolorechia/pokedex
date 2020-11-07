use std::fs;
use std::path::Path;
use std::vec::Vec;
use std::error::Error;

use serde_json;
use mongodb::bson;

// Re-exports modules as pub
pub mod mongo;
pub mod model;
pub mod config;

// Manually export functions from config
pub fn load_config() -> config::Settings {
    config::load_config()
}

pub fn load_pokemon_list(settings: &config::Settings) -> Vec<String> {
    let data_folder = Path::new(&settings.data_folder);
    let pokemon_list_file = data_folder.join(&settings.poke_list_file);

    let contents = fs::read_to_string(pokemon_list_file).expect("Could not open settings file!");

    let deserialized: Vec<&str> = serde_json::from_str(&contents).unwrap();
    let string_vec: Vec<String> = deserialized.iter().map(|x| x.to_string()).collect();
    string_vec
}

pub fn load_pokemon_html_folder(settings: &config::Settings) -> String {
    let data_folder = Path::new(&settings.data_folder);
    let pokemons_html_folder = data_folder.join(&settings.pokemons_html_folder);
    pokemons_html_folder.to_str().unwrap().to_string()
}

pub fn load_pokemon_html(settings: &config::Settings, pokemon: &str) -> Option<String> {
    let pokemon_html_folder = load_pokemon_html_folder(settings);
    let poke_file = pokemon_html_folder + "/" + pokemon + ".html";
    println!("{:?}", poke_file);
    let res = fs::read_to_string(poke_file);
    match res {
        Ok(n) => Some(n),
        _ => None
    }
}

pub async fn save_pokemon_to_mongo(collection: &mongodb::Collection, pokemon: model::Pokemon) -> Result<(), Box<dyn std::error::Error>> {
    let b = bson::to_bson(&pokemon)?;
    let d = b.as_document().unwrap();
    collection.insert_one(d.to_owned(), None).await?;
    Ok(())
}

pub async fn find_pokemon_by_name(collection: &mongodb::Collection, name: &String) -> Result<Option<model::Pokemon>, Box<dyn std::error::Error>> {
    let filter = bson::doc! { "name": name.clone() };
    let cursor = collection.find_one(filter, None).await?;
    let pokemon = match cursor {
        Some(document) => {
            let pokemon: model::Pokemon = bson::from_document(document).unwrap();
            Some(pokemon)
        },
        None => { 
            None
        }
    };
    Ok(pokemon)
}

pub async fn update_pokemon_to_mongo(collection: &mongodb::Collection, pokemon: model::Pokemon) -> Result<(), Box<dyn std::error::Error>> {
    let b = bson::to_bson(&pokemon)?;
    let d = b.as_document().unwrap();
    let filter = bson::doc! { "name": pokemon.name.clone() };
    collection.update_one(filter, d.to_owned(), None).await?;
    Ok(())
}

pub async fn find_pokemons_by_generation(collection: &mongodb::Collection, generation: i32) -> Result<Vec<model::Pokemon>, Box<Error>> {
    let mut result: Vec<model::Pokemon> = vec![];

    Ok(result)
}
