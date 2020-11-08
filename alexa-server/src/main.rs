extern crate alexa_sdk;
extern crate lambda_runtime as lambda;

use alexa_sdk::request::{IntentType, Locale, ReqType};
use alexa_sdk::{Request, Response};
use lambda::{error::HandlerError, lambda, Context};
use std::error::Error;

fn handle_help(_req: &Request) -> Result<Response, HandlerError> {
    Ok(Response::new_simple(
        "hello",
        "to say hello, tell me: say hello to someone",
    ))
}

fn handle_hello(req: &Request) -> Result<Response, HandlerError> {
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
    Ok(Response::reply("hello", "hello world"))
}

fn handle_not_implemented(_req: &Request) -> Result<Response, HandlerError> {
    Ok(Response::reply("not_implemented", "Not implemented :("))
}

fn handle_intent_not_implemented(_req: &Request) -> Result<Response, HandlerError> {
    Ok(Response::reply(
        "not_implemented",
        "Intent not implemented :(",
    ))
}

fn handle_describe_pokemon(req: &Request) -> Result<Response, HandlerError> {
    Ok(Response::reply(
        "not_implemented",
        "Describe not implemented :(",
    ))
}

pub enum CustomIntentType {
    DescribePokemon,
    Unknown,
}

pub fn string_to_intent(intent: String) -> CustomIntentType {
    match intent.as_str() {
        "describePokemon" => CustomIntentType::DescribePokemon,
        _ => CustomIntentType::Unknown,
    }
}

pub fn match_custom_intent(
    intent: CustomIntentType,
    req: &Request,
) -> Result<Response, HandlerError> {
    match intent {
        CustomIntentType::DescribePokemon => handle_describe_pokemon(&req),
        _ => handle_intent_not_implemented(&req),
    }
}

fn handle_intent(req: &Request) -> Result<Response, HandlerError> {
    match req.intent() {
        IntentType::User(customIntent) => match_custom_intent(string_to_intent(customIntent), &req),
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
    lambda!(my_handler);

    Ok(())
}