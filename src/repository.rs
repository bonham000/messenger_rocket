use diesel;
use diesel::prelude::*;

use super::schema::messages;
use super::types::{Message, SavedMessage, InsertableMessage};

/// # Insert new message
/// Method to insert a new message into the database
pub fn insert_message(message: Message, connection: &PgConnection) -> QueryResult<SavedMessage> {
    diesel::insert_into(messages::table)
        .values(&InsertableMessage::from_message(message))
        .get_result(connection)
}