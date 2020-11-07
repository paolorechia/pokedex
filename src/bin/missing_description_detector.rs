use pokedex_scraper::{find_pokemons_by_generation, save_data_report};
use pokedex_scraper::mongo::init_pokemon_collection;
use pokedex_scraper::model::Report;
use std::vec::Vec;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let collection = init_pokemon_collection().await?;
    let first_gen_pokemons = find_pokemons_by_generation(&collection, 1).await?;

    println!("Total number of pokemons: {}", first_gen_pokemons.len());

    let mut missing: Vec<String> = vec![];
    let mut ok: Vec<String> = vec![];
    let mut redirected: Vec<String> = vec![];

    for pokemon in first_gen_pokemons {
        let description = pokemon.description;
        match description.find("editar") {
            Some(_) => {
                println!("Description Redirected: {:?}", description);
                redirected.push(pokemon.name);
            }
            None => {
                if description.replace("\n", "").len() == 0 {
                    println!("Description Missing: {:?}", description);
                    missing.push(pokemon.name);
                } else {
                    println!("Description OK: {:?}", description);
                    ok.push(pokemon.name);
                }
            }
        }
    }

    println!("Total OK descriptions: {:?}", ok.len());
    println!("------ OK ------");
    println!("{:?}", ok);
    println!("Total redirected descriptions: {:?}", redirected.len());
    println!("------ RE ------");
    println!("{:?}", redirected);
    println!("Total missing descriptions: {:?}", missing.len());
    println!("------ MI ------");
    println!("{:?}", missing);

    let report = Report {
        ok,
        redirected,
        missing
    };
    save_data_report(report);

    Ok(())
}
