#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

extern crate dotenv;
use dotenv::dotenv;
use ws::listen;
use std::thread;
use ws::{Message as WSMessage};

mod schema;
mod db;
mod types;
mod routes;
mod service;
mod repository;
mod controllers;

fn main() {
    dotenv().ok();

    // Run WebSocket listener on a separate thread to not block the main server thread
    thread::spawn(|| {
        // Listen on an address and call the closure for each connection
        if let Err(error) = listen("127.0.0.1:3012", |out| {
            // The handler needs to take ownership of out, so we use move
            move |msg: WSMessage| {
                // Parse the message
                let result = service::handle_socket_message(msg);
                match result {
                    Ok(saved_message) => {
                        // Broadcast response if message was valid
                        out.broadcast(format!("{:?}", saved_message))
                    },
                    Err(_) => {
                        // No action if any error
                        println!("Error sending message...");
                        Ok(())
                    }
                }
            }
        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to {:?}", error);
        }
    });

    // Setup Rocket and fire!
    routes::build_server();
}