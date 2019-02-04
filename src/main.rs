#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

use serde::Deserialize;

extern crate dotenv;
use diesel::prelude::*;
use dotenv::dotenv;
use ws::listen;
use std::thread;
use rocket_contrib::json::Json;
use ws::{Message as WSMessage};

mod schema;
mod db;
mod types;
mod routes;
mod service;
mod repository;
mod controllers;

//use types::Message;

fn main() {
    dotenv().ok();

    thread::spawn(|| {
        // Listen on an address and call the closure for each connection
        if let Err(error) = listen("127.0.0.1:3012", |out| {
            // The handler needs to take ownership of out, so we use move
            move |msg: WSMessage| {
                service::handle_socket_message(msg);
                out.broadcast(String::from("Hello from Server!"))
            }
        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to {:?}", error);
        }
    });

    // Setup Rocket and fire!
    routes::build_server();
}