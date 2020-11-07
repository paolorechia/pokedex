extern crate lambda_runtime as lambda;
extern crate alexa_sdk;

use lambda::{lambda, Context, error::HandlerError};
use alexa_sdk::{Request,Response};
use alexa_sdk::request::{IntentType, Locale};
use std::error::Error;

fn handle_help(_req: &Request) -> Result<Response,HandlerError> {
    Ok(Response::new_simple("hello", "to say hello, tell me: say hello to someone"))
}

fn handle_hello(req: &Request) -> Result<Response,HandlerError> {
    let res = match req.locale() {
        Locale::AustralianEnglish => Response::new_simple("hello", "G'day mate"),
        Locale::German => Response::new_simple("hello", "Hallo Welt"),
        Locale::Japanese => Response::new_simple("hello", "こんにちは世界"),
        _ => if let Some(ref s) = req.slot_value("name") {
            Response::new_simple("hello", (String::from("hello ") + s).as_str())
        } else {
            Response::new_simple("hello", "hello world")
        },
    };
    Ok(res)
}

fn handle_cancel(_req: &Request) -> Result<Response,HandlerError> {
    Ok(Response::end())
}

fn my_handler(req: Request, _ctx: Context) -> Result<Response,HandlerError> {
    match req.intent() {
        IntentType::Help => handle_help(&req),
        IntentType::User(_) => handle_hello(&req),
        _ => handle_cancel (&req)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);

    Ok(())
}
