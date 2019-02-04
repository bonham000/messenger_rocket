use rocket;

use super::db;
use super::controllers;

pub fn build_server() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/post_message", routes![controllers::post_message])
        .launch();
}