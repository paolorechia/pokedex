use mongodb::{
    options::ClientOptions,
    Client
};

pub async fn connect() -> Result<Client, Box<dyn std::error::Error>> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub async fn get_db(client: &Client) -> mongodb::Database {
    client.database("pokedex")
}

pub async fn print_db_names(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}

pub async fn init_pokemon_collection() -> Result<mongodb::Collection, Box<dyn std::error::Error>>
{
    let client = connect().await?;
    let db = client.database("pokedex");
    Ok(db.collection("pokemons"))
}
