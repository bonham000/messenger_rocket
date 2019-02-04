use rocket_contrib::json::Json;
use diesel;
use diesel::prelude::*;
use ws::{Message as SocketMessage};
use serde_json::error::{Error as SerdeJsonError};

use super::db::DbConn;
use super::repository;
use super::types::{Message, SavedMessage};

/// # Service method to return recent message history
/// Returns most recent 50 messages
pub fn get_messages(connection: DbConn) -> QueryResult<Vec<SavedMessage>> {
    let result = repository::get_messages(&connection);

    result
}

/// # Service method to save a message
/// Saves a new method to the database
pub fn save_message(message: Json<Message>, connection: DbConn) -> QueryResult<SavedMessage> {
    let result = repository::insert_message(message.into_inner(), &connection);

    result
}

/// # Service method to edit a message
/// Edits an existing message content
pub fn edit_message(message: SavedMessage, connection: DbConn) -> QueryResult<SavedMessage> {
    let result = repository::edit_message(message, &connection);

    result
}

/// # Service method to delete a message
/// Deletes a message by id
pub fn delete_message(id: i32, connection: DbConn) -> QueryResult<usize> {
    let result = repository::delete_message(id, &connection);

    result
}

/// # Handle parsing WebSocket messages
/// Parses message to only forward messages if incoming message is valid
pub fn handle_socket_message(raw_message: SocketMessage) -> Result<SavedMessage, &'static str> {
    let maybe_text= raw_message.as_text();
    match maybe_text {
        Ok(text) => {
            let json_result: Result<SavedMessage, SerdeJsonError> = serde_json::from_str(text);
            match json_result {
                Ok(result) => {
                    println!("Parsed message: {:?}", result);
                    Ok(result)
                },
                Err(_) => {
                    println!("Could not parse result...");
                    Err("Message could not be parsed...")
                }
            }
        },
        Err(_) => {
            println!("Could not parse incoming message...");
            Err("Message could not be parsed...")
        }
    }
}