#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

extern crate dotenv;
use dotenv::dotenv;

mod schema;
mod db;
mod types;
mod routes;
mod service;
mod repository;
mod controllers;

fn main() {
    dotenv().ok();

    // Setup Rocket and fire!
    routes::build_server();
}