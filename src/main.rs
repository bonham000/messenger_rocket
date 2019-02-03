#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;

mod routes;

fn main() {
    rocket::ignite()
        .mount("/post_message", routes![routes::post_message])
        .launch();
}