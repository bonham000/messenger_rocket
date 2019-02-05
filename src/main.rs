#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

extern crate dotenv;
use dotenv::dotenv;

mod controllers;
mod postgres;
mod repository;
mod routes;
mod schema;
mod service;
mod socket;
mod types;

fn main() {
    // Load environment variables
    dotenv().ok();

    // Run the WebSockets listener
    socket::run_socket_listener();

    // Setup Rocket and fire!
    routes::build_server();
}
