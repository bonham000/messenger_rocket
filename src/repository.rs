use diesel;
use diesel::prelude::*;

use super::schema::messages::dsl::*;
use super::types::{Message, SavedMessage, InsertableMessage};

/// # Insert new message
/// Method to insert a new message into the database
pub fn insert_message(new_message: Message, connection: &PgConnection) -> QueryResult<SavedMessage> {
    let result = diesel::insert_into(messages)
        .values(&InsertableMessage::from_message(new_message))
        .get_result(connection);

    result
}

/// # Edit a message
/// Method to edit an existing message
pub fn edit_message(message_edit: SavedMessage, connection: &PgConnection) -> QueryResult<SavedMessage> {
    let result = diesel::update(messages.filter(id.eq(message_edit.id)))
        .set(message.eq(message_edit.message))
        .get_result(connection);

    result
}

/// # Delete a message
/// Handle deleting a message by id
pub fn delete_message(message_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(messages.find(message_id)).execute(connection)
}

/// # Get messages
/// Method to return recent messages
pub fn get_messages(connection: &PgConnection) -> QueryResult<Vec<SavedMessage>> {
    let limit = 50;
    let result = messages.limit(limit).load::<SavedMessage>(&*connection);

    result
}