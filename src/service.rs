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