/// This binary reads data from MongoDB and dumps to a Redis Lambda Store
/// Since we're dealing with a low amount of data, this is possible for free :)

use std::error::Error;
use std::boxed::Box;
use redis::{create_connection, test_dump_pokemon};
use pokedex_scraper::mongo::{init_pokemon_collection};
use pokedex_scraper::find_pokemons_by_generation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mongo = init_pokemon_collection().await?;
    let pokemons = find_pokemons_by_generation(&mongo, 1).await?;
    let mut redis_connection = create_connection()?;
    for pokemon in pokemons {
        let redis_pokemon = redis::Pokemon {
            name: pokemon.name,
            description: pokemon.description,
            origin: pokemon.origin,
            name_origin: pokemon.name_origin,
            evolution: pokemon.evolution,
            category: pokemon.category,
            height: pokemon.height,
            weight: pokemon.weight,
            pokemon_types: pokemon.pokemon_types,
            generation: pokemon.generation
        };
        test_dump_pokemon(&mut redis_connection, redis_pokemon)?;
    }
    Ok(())
}
