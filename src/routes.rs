use rocket_contrib::json::Json;
use diesel;
use diesel::prelude::*;

use super::schema::messages;
use super::db::DbConn;

#[post("/", format="json", data = "<message>")]
pub fn post_message(message: Json<Message>, connection: DbConn) -> Json<Status> {
    println!("Message received: {:?}", message);

    let saved_message = insert_message(message.into_inner(), &connection);

    println!("Message saved! {:?}", saved_message);

    Json(Status {
        status: String::from("OK"),
    })
}

pub fn insert_message(message: Message, connection: &PgConnection) -> QueryResult<SavedMessage> {
    diesel::insert_into(messages::table)
        .values(&InsertableMessage::from_message(message))
        .get_result(connection)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message: String,
    author: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    status: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "messages"]
pub struct SavedMessage {
    pub id: i32,
    pub message: String,
    pub author: String,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct InsertableMessage {
    pub message: String,
    pub author: String,
}

impl InsertableMessage {
    fn from_message(message: Message) -> InsertableMessage {
        InsertableMessage {
            message: (*message.message).to_string(),
            author: (*message.author.name).to_string(),
        }
    }
}