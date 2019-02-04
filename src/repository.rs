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

/// # Get messages
/// Returns most recent 50 messages
pub fn get_messages(connection: &PgConnection) -> QueryResult<Vec<SavedMessage>> {
    let limit = 50;
    let result = messages::table.limit(limit).load::<SavedMessage>(&*connection);

    result
}