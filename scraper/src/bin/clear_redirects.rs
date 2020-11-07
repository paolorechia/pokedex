use pokedex_scraper::{find_pokemons_by_generation, update_pokemon_to_mongo};
use pokedex_scraper::mongo::init_pokemon_collection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let collection = init_pokemon_collection().await?;
    let first_gen_pokemons = find_pokemons_by_generation(&collection, 1).await?;

    let mut redirects_cleared = 0;
    for mut pokemon in first_gen_pokemons {
        let description = pokemon.description;
        match description.find("editar") {
            Some(_) => {
                pokemon.description = "".to_string();
                update_pokemon_to_mongo(&collection, pokemon).await?;
                redirects_cleared += 1;
            }
            None => ()
        }
    }
    println!("Cleared {} redirect descriptions", redirects_cleared);
    Ok(())
}
