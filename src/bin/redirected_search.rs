use std::boxed::Box;
use std::result::Result;
use std::time::Duration;
use reqwest::StatusCode;
use std::thread;
use std::fs;
use std::path::Path;

use pokedex_scraper::{load_config, load_data_report};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = load_config();
    let report = load_data_report();
    for pokemon in report.redirected {
        let url = settings.poke_root_url.clone() + &pokemon.to_string() + "_(Pokémon)";
        println!("{}", url);
        let resp = reqwest::get(&url).await?;
        let status = resp.status();
        println!("Returned: {:?}", status);
        if status == StatusCode::OK {
            println!("OK! Let's save it to a file.");
            let body = resp.text().await?;
            let data_folder = Path::new(&settings.data_folder);
            let redirected_folder = data_folder.join(&settings.redirected_folder);
            let filepath = redirected_folder.join(pokemon + ".html");
            println!("Writing to {:?}", filepath);
            fs::write(filepath, body).expect("Could not write HTML file.");
        }
        thread::sleep(Duration::from_millis(1000));
    }
    Ok(())
}
