use rocket_contrib::json::Json;
use diesel;
use diesel::prelude::*;

use super::db::DbConn;
use super::repository;
use super::types::{Message, SavedMessage};

/// # Service method to save a message
/// Saves a new method to the database
pub fn save_message(message: Json<Message>, connection: DbConn) -> QueryResult<SavedMessage> {
    let result = repository::insert_message(message.into_inner(), &connection);
    println!("Message saved! {:?}", result);

    result
}

/// # Service method to return recent message history
/// Returns most recent 50 messages
pub fn get_messages(connection: DbConn) -> QueryResult<Vec<SavedMessage>> {
    let result = repository::get_messages(&connection);
    println!("Messages loaded! {:?}", result);

    result
}