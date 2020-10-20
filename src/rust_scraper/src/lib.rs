use std::fs;
use std::path::Path;
use std::vec::Vec;

use serde_json;

mod config;

#[derive(Debug)]
struct Pokemon {
    origin: String,
    name_origin: String,
    evolution: String,
    category: String,
    height: String,
    weight: String,
    pokemon_types: Vec<String>,
    generation: i32
}

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
    let res = fs::read_to_string(poke_file);
    match res {
        Ok(n) => Some(n),
        _ => None
    }
}
