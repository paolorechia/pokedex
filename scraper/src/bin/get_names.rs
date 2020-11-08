use std::error::Error;
use std::boxed::Box;
use pokedex_scraper::mongo::{init_pokemon_collection};
use pokedex_scraper::find_pokemons_by_generation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mongo = init_pokemon_collection().await?;
    let pokemons = find_pokemons_by_generation(&mongo, 1).await?;
    for pokemon in pokemons {
        println!("{},", pokemon.name);
    }
    Ok(())
}
