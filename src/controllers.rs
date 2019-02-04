use rocket_contrib::json::Json;

use super::db::DbConn;
use super::service;
use super::types::{Message, Status};

/// # POST message handler
/// Handler for posting a new chat message
#[post("/post_message", format="json", data = "<message>")]
pub fn save_message(message: Json<Message>, connection: DbConn) -> Json<Status> {
    println!("Message received: {:?}", message);

    let saved_message = service::save_message(message, connection);

    match saved_message {
        Ok(m) => {
            println!("Message saved! {:?}", m);
            Json(Status {
                status: String::from("OK! Message saved successfully."),
            })
        },
        _ => {
            println!("Error saving message");
            Json(Status {
                status: String::from("Error! Message could not be saved."),
            })
        }
    }
}

// TODO:
// Add Read Endpoint (all message history)
// Add Edit Endpoint
// Add Delete Endpoint
