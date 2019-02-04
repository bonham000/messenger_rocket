use rocket_contrib::json::Json;

use super::db::DbConn;
use super::service;
use super::types::{Message, SavedMessage, StatusResponse};

/// # POST message handler
/// Handler for posting a new chat message
#[post("/post_message", format="json", data = "<message>")]
pub fn save_message(message: Json<Message>, connection: DbConn) -> Json<StatusResponse> {
    println!("Message received: {:?}", message);

    let saved_message = service::save_message(message, connection);

    match saved_message {
        Ok(m) => {
            println!("Message saved! {:?}", m);
            Json(StatusResponse {
                status: String::from("OK! Message saved successfully."),
            })
        },
        _ => {
            println!("Error saving message");
            Json(StatusResponse {
                status: String::from("Error! Message could not be saved."),
            })
        }
    }
}

/// # GET message history handler
/// Returns most recent 50 messages
#[get("/messages")]
pub fn get_messages(connection: DbConn) -> Result<Json<Vec<SavedMessage>>, String> {
    let messages = service::get_messages(connection);

    match messages {
        Ok(m) => {
            println!("Returning message history to client!");
            Ok(Json(m))
        },
        Err(_) => {
            println!("Error loading message history...");
            Err(String::from("Error loading message history..."))
        }
    }
}

// TODO:
// Add Edit Endpoint
// Add Delete Endpoint
