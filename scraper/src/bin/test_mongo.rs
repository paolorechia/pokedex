use pokedex_scraper::mongo::{connect, print_db_names};
use model::{Pokemon};
use mongodb::bson;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = connect().await?;
    print_db_names(&client).await?;
    let db = client.database("pokedex");
    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    let collection = db.collection("pokemons");
    let pokemon = Pokemon {
        id: None,
        name: "".to_string(),
        description: "".to_string(),
        origin: "Test".to_string(),
        name_origin: "Test".to_string(),
        evolution: "test".to_string(),
        category: "".to_string(),
        height: "".to_string(),
        weight: "".to_string(),
        pokemon_types: vec!["A".to_string(), "B".to_string()],
        generation: -1
    };
    let b = bson::to_bson(&pokemon)?;
    let d = b.as_document().unwrap();
    let r = collection.insert_one(d.to_owned(), None).await?;

    println!("{:?}", r.inserted_id.as_object_id().expect("Oh no"));
    Ok(())
}
