use rocket;

use super::db;
use super::controllers;

/// # Builder for Rocket server and routes
/// Initializes database connection pool and route handlers and launches Rocket
pub fn build_server() {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![controllers::save_message, controllers::get_messages])
        .launch();
}