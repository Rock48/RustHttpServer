#![allow(dead_code)]
#[macro_use]
extern crate io_error;

/** Shorthand for Some(String::From(x)) */
macro_rules! some_str {
    ($x: expr) => {
        Some(Str!($x))
    };
}
/** Shorthand for String::From(x) */
macro_rules! Str {
    ($x: expr) => {
        String::from($x)
    };
}

mod website_handler;
mod http;
use http::Server;
use website_handler::WebsiteHandler;
use std::env;

fn main() {
    let mut server = Server::new("127.0.0.1".to_string(), 8080);
    let default_public = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let pub_dir = env::var("default_public").unwrap_or(default_public);
    println!("Public path set to: {}", pub_dir);
    let handler = WebsiteHandler::new(pub_dir);
    server.run(handler);
}

