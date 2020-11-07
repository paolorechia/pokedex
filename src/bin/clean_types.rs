use pokedex_scraper::mongo::init_pokemon_collection;
use pokedex_scraper::{find_pokemons_by_generation, update_pokemon_to_mongo};
use std::boxed::Box;
use std::error::Error;
use std::result::Result;
use std::vec::Vec;

fn cleanup_type(types: Vec<String>) -> Vec<String> {
    let bad_input = "|inline-block|none";
    let mut cleaned_types: Vec<String> = vec![];
    for type_ in types {
        let clean_str = type_
            .replace(bad_input, "")
            .replace("\n", "")
            .replace("{{", "")
            .replace("}}", "");
        let clean_types = clean_str.split_whitespace();
        for clean_type in clean_types {
            cleaned_types.push(clean_type.trim().to_string());
        }
    }
    if cleaned_types.len() == 0 {
        cleaned_types.push("".to_string());
    }
    cleaned_types
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let collection = init_pokemon_collection().await?;
    let pokemons = find_pokemons_by_generation(&collection, 1).await?;
    for mut pokemon in pokemons {
        pokemon.pokemon_types= cleanup_type(pokemon.pokemon_types);
        update_pokemon_to_mongo(&collection, pokemon).await?;
    }
    Ok(())
}

#[test]
fn test_cleanup_type_1() {
    let test_input = vec![String::from(
        "{{Venenoso|inline-block|none}}  {{Terrestre}}\n",
    )];
    let test_output: Vec<String> = vec![String::from("Venenoso"), String::from("Terrestre")];
    assert_eq!(cleanup_type(test_input), test_output);
}
#[test]
fn test_cleanup_type_2() {
    let test_input = vec![String::from("{{Normal|inline-block|none}}\n")];
    let test_output: Vec<String> = vec![String::from("Normal")];
    assert_eq!(cleanup_type(test_input), test_output);
}
#[test]
fn test_cleanup_type_3() {
    let test_input = vec![String::from("")];
    let test_output: Vec<String> = vec![String::from("")];
    assert_eq!(cleanup_type(test_input), test_output);
}

#[test]
fn test_cleanup_type_4() {
    let test_input = vec![String::from("Fogo"), String::from("Agua")];
    let test_output: Vec<String> = vec![String::from("Fogo"), String::from("Agua")];
    assert_eq!(cleanup_type(test_input), test_output);
}
