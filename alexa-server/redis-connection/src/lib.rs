use std::env;

extern crate redis;
use redis::Commands;

// serde_redis does not have a serialization
// algorithm implemented, so we'll resort to plain
// JSON

use serde::{Deserialize, Serialize};

pub struct RedisConfig {
    password: String,
    hostname: String,
    port: String,
}

// Redeclare Pokemon struct without Mongo BSON
// This avoids excessive dependencies for the lambda
// that will use this crate.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
    pub origin: String,
    pub name_origin: String,
    pub evolution: String,
    pub category: String,
    pub height: String,
    pub weight: String,
    pub pokemon_types: Vec<String>,
    pub generation: i32,
}

pub fn read_credentials_from_env() -> RedisConfig {
    // If we're missing these  variables, it's ok to panick!
    // There's nothing we can do without credentials.
    let hostname = env::var("REDIS_HOSTNAME").expect("Missing hostname environment variable.");
    let port = env::var("REDIS_PORT").expect("Missing port environment variable.");
    let password = env::var("REDIS_PASSWORD").expect("Missing password environment variable.");
    RedisConfig {
        password,
        hostname,
        port,
    }
}

pub fn config_to_url(config: RedisConfig) -> String {
    // Desired URL is:
    // redis://[<username>][:<passwd>@]<hostname>[:port][/<db>]
    // However for lambda store, we don't have any value for username...
    // redis://[<username>][:<passwd>@]<hostname>[:port][/<db>]

    "redis://:".to_string() + &config.password + "@" + &config.hostname + ":" + &config.port
}

pub fn create_connection() -> redis::RedisResult<redis::Connection> {
    let config = read_credentials_from_env();
    let url = config_to_url(config);
    println!("URL: {}", url);
    let client = redis::Client::open(url)?;
    let con = client.get_connection()?;
    println!("Connected!");
    Ok(con)
}

pub fn hello_world(con: &mut redis::Connection) -> redis::RedisResult<()> {
    println!("Test Set key");
    let _: () = con.set("hello_world", 43)?;
    println!("Test Get key");
    let x: i32 = con.get("hello_world")?;
    println!("Hello world executed, redis replied: {:?}", x);
    Ok(())
}

pub fn save_pokemon(con: &mut redis::Connection, pokemon: Pokemon) -> redis::RedisResult<()> {
    let pokemon_string = serde_json::to_string(&pokemon).expect("Failed to serialize pokemon, panick!!");
    con.set(pokemon.name, pokemon_string)?;
    Ok(())
}

pub fn load_pokemon(
    con: &mut redis::Connection,
    pokemon_name: String,
) -> redis::RedisResult<Pokemon> {
    let pokemon_string: String = con.get(pokemon_name)?;

    let pokemon =
        serde_json::from_str(&pokemon_string).expect("Could not get result from Redis, panick!!");
    Ok(pokemon)
}

pub fn test_dump_pokemon(mut con: &mut redis::Connection, pokemon: Pokemon) -> redis::RedisResult<()> {
    println!("Let's test save and load for a pokemon...");
    let name = pokemon.name.clone();
    println!("Name: {}", name);
    println!("Saving...");
    save_pokemon(&mut con, pokemon)?;
    println!("Loading...");
    let pokemon: Pokemon = load_pokemon(&mut con, name)?;
    println!("Found in Redis: {:?}", pokemon);
    Ok(())
}
