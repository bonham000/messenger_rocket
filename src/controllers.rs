use rocket_contrib::json::Json;

use super::db::DbConn;
use super::service;
use super::types::{Message, Status};

#[post("/", format="json", data = "<message>")]
pub fn post_message(message: Json<Message>, connection: DbConn) -> Json<Status> {
    println!("Message received: {:?}", message);

    let saved_message = service::post_message(message, connection);

    println!("Message saved! {:?}", saved_message);

    Json(Status {
        status: String::from("OK"),
    })
}