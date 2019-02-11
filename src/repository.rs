use diesel;
use diesel::prelude::*;

use super::schema::messages::dsl::*;
use super::types::{InsertableMessage, Message, SavedMessage};

/// # Get messages
/// Method to return recent messages
pub fn get_messages(limit: i64, connection: &PgConnection) -> QueryResult<Vec<SavedMessage>> {
    messages.limit(limit).order(id).load::<SavedMessage>(&*connection)
}

/// # Insert new message
/// Method to insert a new message into the database
pub fn save_message(new_message: Message, connection: &PgConnection) -> QueryResult<SavedMessage> {
    diesel::insert_into(messages)
        .values(&InsertableMessage::from_message(new_message))
        .get_result(connection)
}

/// # Edit a message
/// Method to edit an existing message
pub fn edit_message(
    message_edit: SavedMessage,
    connection: &PgConnection,
) -> QueryResult<SavedMessage> {
    diesel::update(messages.filter(id.eq(message_edit.id)))
        .set(message.eq(message_edit.message))
        .get_result(connection)
}

/// # Delete a message
/// Handle deleting a message by id
pub fn delete_message(message_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(messages.find(message_id)).execute(connection)
}

/// # Delete all messages
/// Handle deleting all messages
pub fn delete_all(connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(messages).execute(connection)
}
