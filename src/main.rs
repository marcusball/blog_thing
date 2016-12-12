#[macro_use]
extern crate iron;
extern crate params;
extern crate router;
extern crate logger;
extern crate iron_error_logger;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{
        foreign_links {
            IronHttpError(::iron::error::HttpError);
        }
    }
}

use iron::prelude::*;
use iron::status;
use router::Router;
use logger::Logger;
use logger::format::Format;
use iron_error_logger::ErrorLogger;
use dotenv::dotenv;
use errors::*;
use std::env;


static FORMAT: &'static str = "[{request-time}] {method} {uri} {status} ({response-time})";

/// A basic "hello world" handler for a request to "/"
pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello, World!")))
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}



// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<()> {
    dotenv().ok();
    env_logger::init().unwrap();

    let format = Format::new(FORMAT);

    let (logger_before, logger_after) = Logger::new(Some(format.unwrap()));
    let error_logger = ErrorLogger::new();

    let mut router = Router::new();           // Alternative syntax:
    router.get("/", index, "index");        // let router = router!(index: get "/" => handler,
    // router.get("/rewards", handlers::get_rewards, "get rewards");
    // router.post("/rewards", handlers::add_rewards, "add rewards");
    // router.delete("/rewards", handlers::delete_rewards, "delete rewards");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
    chain.link_after(error_logger);

    match Iron::new(chain).http("0.0.0.0:3000") {
        Ok(listening) => {
            info!("Server running on {}", listening.socket);
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
