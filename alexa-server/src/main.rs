// TODO: this code needs some serious refactoring :)
// TODO: split code across a few different files

extern crate alexa_sdk;
extern crate lambda_runtime as lambda;
extern crate redis;
extern crate redis_client;

use alexa_sdk::request::{IntentType, Locale, ReqType};
use alexa_sdk::{Request, Response};
use lambda::{error::HandlerError, lambda, Context};
use redis_client::{create_connection, load_pokemon, Pokemon};
use std::error::Error;

// TODO: Replace this with proper shared variable (Maybe Arc?)
static mut REDIS_CONNECTION: Option<redis::Connection> = None;

// TODO: implement handle help
fn _handle_help(_req: &Request) -> Result<Response, HandlerError> {
    Ok(Response::new_simple(
        "hello",
        "to say hello, tell me: say hello to someone",
    ))
}

// TODO: implment proper locale handler
fn _handle_locale(req: &Request) -> Result<Response, HandlerError> {
    let res = match req.locale() {
        Locale::AustralianEnglish => Response::new_simple("hello", "G'day mate"),
        Locale::German => Response::new_simple("hello", "Hallo Welt"),
        Locale::Japanese => Response::new_simple("hello", "こんにちは世界"),
        _ => {
            if let Some(ref s) = req.slot_value("name") {
                Response::new_simple("hello", (String::from("hello ") + s).as_str())
            } else {
                Response::new_simple("hello", "hello world")
            }
        }
    };
    Ok(res)
}

fn handle_cancel(_req: &Request) -> Result<Response, HandlerError> {
    Ok(Response::end())
}

fn handle_launch(_req: &Request) -> Result<Response, HandlerError> {
    Ok(Response::reply("hello", "Olá, bem vindo a Pokedex Primeira Geração. Aqui você pode perguntar a respeito de pokemons da primeira geração. Tente começar com 'descreva Pikachu'."))
}

fn unavailable_feature() -> Response {
    Response::reply(
        "not_implemented",
        "Função indisponível. Contate o desenvolvendor.",
    )
}

fn handle_not_implemented(_req: &Request) -> Result<Response, HandlerError> {
    Ok(unavailable_feature())
}

fn handle_intent_not_implemented(_req: &Request) -> Result<Response, HandlerError> {
    Ok(Response::reply(
        "not_implemented",
        "Intenção indisponível. Contate o desenvolvendor.",
    ))
}

fn description_extractor(pokemon: Pokemon) -> Response {
    if pokemon.description.len() > 0 {
        Response::reply("descricao", &pokemon.description)
    } else {
        Response::reply("sem_descricao", "Pokemon sem descrição disponível.")
    }
}

fn height_extractor(pokemon: Pokemon) -> Response {
    if pokemon.height.len() > 0 {
        let height = "O pokemon ".to_string()
            + &pokemon.name
            + " tem "
            + &pokemon.height.replace("m", "")
            + " metros de altura.";
        Response::reply("altura", &height)
    } else {
        Response::reply("sem_altura", "Pokemon sem informação de altura disponível.")
    }
}

fn weight_extractor(pokemon: Pokemon) -> Response {
    if pokemon.weight.len() > 0 {
        let weight = "O pokemon ".to_string()
            + &pokemon.name
            + " tem "
            + &pokemon.weight.replace("kg", "")
            + " kilogramas de massa.";
        Response::reply("altura", &weight)
    } else {
        Response::reply("sem_peso", "Pokemon sem informação de peso disponível.")
    }
}

fn type_extractor(pokemon: Pokemon) -> Response {
    let types: Vec<String> = pokemon.pokemon_types;
    if types.len() > 0 && types[0].len() > 0 {
        let mut tipo = "".to_string();
        if types.len() == 1 {
            tipo = "Tipo ".to_string() + &types[0];
        }
        if types.len() == 2 {
            tipo = "Tipos ".to_string() + &types[0] + " e " + &types[1];
        }
        Response::reply("tipo", &tipo)
    } else {
        Response::reply("sem_tipo", "Pokemon sem informação de tipo.")
    }
}

fn evolution_extractor(pokemon: Pokemon) -> Response {
    if pokemon.evolution.len() > 0 {
        Response::reply("evolução", &pokemon.evolution)
    } else {
        Response::reply("sem_evolucao", "Pokemon sem informação de evolução.")
    }
}

// TODO: Implement
fn origin_extractor(pokemon: Pokemon) -> Response {
    if pokemon.origin.len() > 0 {
        Response::reply("origem", &pokemon.origin)
    } else {
        Response::reply("sem_origem", "Pokemon sem informação de origem.")
    }
}

// TODO: Implement
fn name_origin_extractor(pokemon: Pokemon) -> Response {
    if pokemon.name_origin.len() > 0 {
        Response::reply("origem", &pokemon.name_origin)
    } else {
        Response::reply("sem_origem_nome", "Pokemon sem informação de origem do nome.")
    }
}

fn handle_pokemon_intent(
    req: &Request,
    extractor: fn(Pokemon) -> Response,
) -> Result<Response, HandlerError> {
    // TODO: reuse existing redis connection instead of recreating one every time.
    // TODO: extract all response strings to a proper place.
    let redis = create_connection();
    match redis {
        Ok(mut redis) => {
            let pokemon_name = req.slot_value("Pokemon");
            match pokemon_name {
                Some(name) => {
                    let pokemon = load_pokemon(&mut redis, name.to_lowercase()).unwrap();
                    Ok(extractor(pokemon))
                }
                None => Ok(Response::reply("sem_pokemon", "Pokemon não identificado.")),
            }
        }
        // TODO: attempt to reconnect if connection is lost
        Err(_) => Ok(Response::reply(
            "banco_de_dados_indisponivel",
            "Banco de dados indisponível. Tente mais tarde ou contate o desenvolvedor.",
        )),
    }
}

pub enum CustomIntentType {
    DescribePokemon,
    PokemonHeight,
    PokemonWeight,
    PokemonTypes,
    PokemonEvolution,
    PokemonOrigin,
    PokemonNameOrigin,
    Unknown,
}

pub fn string_to_intent(intent: String) -> CustomIntentType {
    match intent.as_str() {
        "describePokemon" => CustomIntentType::DescribePokemon,
        "PokemonHeight" => CustomIntentType::PokemonHeight,
        "PokemonWeight" => CustomIntentType::PokemonWeight,
        "PokemonTypes" => CustomIntentType::PokemonTypes,
        "PokemonEvolution" => CustomIntentType::PokemonEvolution,
        "PokemonOrigin" => CustomIntentType::PokemonOrigin,
        "PokemonNameOrigin" => CustomIntentType::PokemonNameOrigin,
        _ => CustomIntentType::Unknown,
    }
}

pub fn match_custom_intent(
    intent: CustomIntentType,
    req: &Request,
) -> Result<Response, HandlerError> {
    match intent {
        CustomIntentType::DescribePokemon => handle_pokemon_intent(&req, description_extractor),
        CustomIntentType::PokemonHeight => handle_pokemon_intent(&req, height_extractor),
        CustomIntentType::PokemonWeight => handle_pokemon_intent(&req, weight_extractor),
        CustomIntentType::PokemonTypes => handle_pokemon_intent(&req, type_extractor),
        CustomIntentType::PokemonEvolution => handle_pokemon_intent(&req, evolution_extractor),
        CustomIntentType::PokemonOrigin=> handle_pokemon_intent(&req, origin_extractor),
        CustomIntentType::PokemonNameOrigin=> handle_pokemon_intent(&req, name_origin_extractor),
        _ => handle_intent_not_implemented(&req),
    }
}

fn handle_intent(req: &Request) -> Result<Response, HandlerError> {
    match req.intent() {
        IntentType::User(custom_intent) => match_custom_intent(string_to_intent(custom_intent), &req),
        _ => handle_cancel(&req),
    }
}

fn my_handler(req: Request, _ctx: Context) -> Result<Response, HandlerError> {
    match req.reqtype() {
        ReqType::LaunchRequest => handle_launch(&req),
        ReqType::IntentRequest => handle_intent(&req),
        _ => handle_not_implemented(&req),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: remove this block once proper shared variable is used.
    unsafe {
        let redis = create_connection();
        match redis {
            Ok(redis) => {
                println!("Connected to Redis succesfully!");
                REDIS_CONNECTION = Some(redis);
            }
            Err(e) => {
                println!("Error connecting to Redis: {:?}", e);
            }
        }
    }
    lambda!(my_handler);
    Ok(())
}
