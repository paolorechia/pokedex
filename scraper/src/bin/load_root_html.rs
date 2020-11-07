use std::boxed::Box;
use std::fs;
use std::path::Path;
use std::result::Result;

use pokedex_scraper::load_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = load_config();

    let data_folder = Path::new(&settings.data_folder);
    let pokemon_list_html = data_folder.join(&settings.root_html_file);
    let resp = reqwest::get(&settings.poke_list_url).await?;
    let body = resp.text().await?;
    fs::write(pokemon_list_html, body).expect("Could not write HTML file.");

    println!("Saved! :)");
    Ok(())
}
