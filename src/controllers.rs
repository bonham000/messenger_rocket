use rocket_contrib::json::Json;

use super::db::DbConn;
use super::service;
use super::types::{Message, SavedMessage, StatusResponse};

/// # POST message handler
/// Handler for posting a new chat message
#[post("/message", format="json", data = "<message>")]
pub fn save_message(message: Json<Message>, connection: DbConn) -> Result<Json<SavedMessage>, String> {
    println!("New message received: {:?}", message);

    let saved_message = service::save_message(message, connection);

    match saved_message {
        Ok(m) => {
            println!("Message saved! {:?}", m);
            Ok(Json(m))
        },
        _ => {
            println!("Error saving message");
            Err(String::from("Could not save message!"))
        }
    }
}

/// # PUT message edit handler
/// Edits a message content for an existing message
#[put("/message", format = "json", data = "<message>")]
pub fn edit_message(message: Json<SavedMessage>, connection: DbConn) -> Result<Json<SavedMessage>, String> {
    println!("Editing existing message!");

    let result = service::edit_message(message.into_inner(), connection);

    match result {
        Ok(m) => {
            Ok(Json(m))
        },
        _ => {
            Err(String::from("Could not edit message!"))
        }
    }
}

/// # DELETE an existing message
/// Deletes a message given the id
#[delete("/message/<id>")]
pub fn delete_message(id: i32, connection: DbConn) -> Result<Json<StatusResponse>, String> {
    let result = service::delete_message(id, connection);

    match result {
        Ok(_) => {
            Ok(Json(StatusResponse {
                status: String::from("Message deleted!")
            }))
        },
        _ => {
            Err(String::from("Could not delete message"))
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

//pub fn parse_socket_message(msg: &str) {
//    println!("msg: {}", msg);
//    let message = serde_json::from_str(msg).un_wrap();
//}