use rocket;

use super::controllers;
use super::postgres;

/// # Builder for Rocket server and routes
/// Initializes database connection pool and route handlers and launches Rocket
pub fn build_server() {
    rocket::ignite()
        .manage(postgres::init_pool())
        .mount(
            "/",
            routes![
                controllers::health_check,
                controllers::get_messages,
                controllers::save_message,
                controllers::edit_message,
                controllers::delete_message
            ],
        )
        .launch();
}
