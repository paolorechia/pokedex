use pokedex_scraper::load_config;
use select::document::Document;
use select::predicate::{Class, Name};
use std::boxed::Box;
use std::result::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = load_config();

    /*
    let data_folder = Path::new(&settings.data_folder);
    */

    let document = Document::from(include_str!("../../data/first_gen.html"));

    let wikitable = document.find(Class("wikitable")).next().unwrap();

    // let selector = Selector::parse(".wikitable > tbody > tr > th").unwrap();
    let descendants = wikitable.children();
    for d in descendants {
        // d.descendants();
        let name = d.name();
        if name.is_some() && name.unwrap() == "tbody" {
            println!("{:?}", name.unwrap());
            let rows = d.children();
            for row in rows {
                let is_row = row.name().is_some() && row.name().unwrap() == "tr";
                if is_row {
                    let th = row.find(Name("th")).next().unwrap();
                    let mut name = String::from(th.text().replace("\n", ""));
                    name = name.trim().to_string();
                    println!("{:?}", name);
                }
            }
        }
    }

    Ok(())
}
