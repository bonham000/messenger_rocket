#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

extern crate dotenv;
use dotenv::dotenv;

pub mod schema;
pub mod routes;
pub mod db;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(db::init_pool())
        .mount("/post_message", routes![routes::post_message])
        .launch();
}