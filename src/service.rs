use diesel;
use diesel::prelude::*;
use rocket_contrib::json::Json;

use super::postgres::DbConn;
use super::repository;
use super::types::{Message, SavedMessage};

/// # Service method to return recent message history
/// Returns most recent message history
pub fn get_messages(limit: i64, connection: DbConn) -> QueryResult<Vec<SavedMessage>> {
    repository::get_messages(limit, &connection)
}

/// # Service method to save a message
/// Saves a new method to the database
pub fn save_message(message: Json<Message>, connection: DbConn) -> QueryResult<SavedMessage> {
    repository::save_message(message.into_inner(), &connection)
}

/// # Service method to edit a message
/// Edits an existing message content
pub fn edit_message(message: SavedMessage, connection: DbConn) -> QueryResult<SavedMessage> {
    repository::edit_message(message, &connection)
}

/// # Service method to delete a message
/// Deletes a message by id
pub fn delete_message(id: i32, connection: DbConn) -> QueryResult<usize> {
    repository::delete_message(id, &connection)
}

/// # Service method to delete all messages
/// Deletes all existing messages
pub fn delete_all(connection: DbConn) -> QueryResult<usize> {
    repository::delete_all(&connection)
}
