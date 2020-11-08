use std::env;

extern crate redis;
use redis::Commands;

pub struct RedisConfig {
    password: String,
    hostname: String,
    port: String,
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

pub fn hello_world(con: &mut redis::Connection) -> redis::RedisResult<()>{
 
    println!("Test Set key");
    let _ : () = con.set("hello_world", 43)?;
    println!("Test Get key");
    let x: i32 = con.get("hello_world")?;
    println!("Hello world executed, redis replied: {:?}", x);
    Ok(())
}
