use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub data_folder: String,
    pub poke_root_url: String,
    pub poke_list_url: String,
    pub poke_db: String,
    pub scraping_delay_in_seconds: i32,
    pub root_html_file: String,
    pub pokemons_html_folder: String,
    pub poke_list_file: String,
}

pub fn load_config() -> Settings {
    let contents =
        fs::read_to_string("config/settings.json").expect("Could not open settings file!");

    let deserialized: Settings = serde_json::from_str(&contents).unwrap();
    let x = deserialized;
    return x;
}

