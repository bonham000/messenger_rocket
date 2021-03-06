use rocket::http::Status;
use rocket::Response;
use rocket_contrib::json::Json;

use super::postgres::DbConn;
use super::service;
use super::types::{Message, SavedMessage, StatusResponse};

/// # Simple health check endpoint
#[get("/rocket")]
pub fn health_check() -> String {
    String::from("Hello from Messenger Rocket server")
}

/// # GET message history handler
/// Returns most recent messages
#[get("/messages?<limit>")]
pub fn get_messages(
    limit: Option<i64>,
    connection: DbConn,
) -> Result<Json<Vec<SavedMessage>>, Response<'static>> {
    let default_limit = 50;
    let page_limit = match limit {
        Some(value) => value,
        None => default_limit,
    };

    let result = service::get_messages(page_limit, connection);

    match result {
        Ok(messages) => {
            println!("Returning message history to client");
            Ok(Json(messages))
        }
        Err(e) => {
            println!("Error getting message history: {:?}", e);
            Err(get_failure_status())
        }
    }
}

/// # POST message handler
/// Handler for posting a new chat message
#[post("/message", format = "json", data = "<message>")]
pub fn save_message(
    message: Json<Message>,
    connection: DbConn,
) -> Result<Json<SavedMessage>, Response<'static>> {
    let result = service::save_message(message, connection);

    match result {
        Ok(saved_message) => {
            println!("New message saved: {:?}", saved_message);
            Ok(Json(saved_message))
        }
        Err(e) => {
            println!("Error saving message, {:?}", e);
            Err(get_failure_status())
        }
    }
}

/// # PUT message edit handler
/// Edits a message content for an existing message
#[put("/message", format = "json", data = "<message>")]
pub fn edit_message(
    message: Json<SavedMessage>,
    connection: DbConn,
) -> Result<Json<SavedMessage>, Response<'static>> {
    let result = service::edit_message(message.into_inner(), connection);

    match result {
        Ok(saved_message) => {
            println!("Message edited successfully: {:?}", saved_message);
            Ok(Json(saved_message))
        }
        Err(e) => {
            println!("Error editing message: {:?}", e);
            Err(get_failure_status())
        }
    }
}

/// # DELETE an existing message
/// Deletes a message given the id
#[delete("/message/<id>")]
pub fn delete_message(
    id: i32,
    connection: DbConn,
) -> Result<Json<StatusResponse>, Response<'static>> {
    let result = service::delete_message(id, connection);

    match result {
        Ok(_) => {
            println!("Message deleted successfully, id: {:?}", id);
            Ok(Json(StatusResponse {
                status: String::from("Message deleted!"),
            }))
        }
        Err(e) => {
            println!("Error deleting message: {:?}", e);
            Err(get_failure_status())
        }
    }
}

/// # DELETE all existing messages
/// Deletes all messages that exist
/// e.g. curl -X DELETE https://shrouded-coast-91311.herokuapp.com/admin/delete
#[delete("/admin/delete")]
pub fn delete_all(connection: DbConn) -> Result<Json<StatusResponse>, Response<'static>> {
    let result = service::delete_all(connection);

    match result {
        Ok(_) => Ok(Json(StatusResponse {
            status: String::from("All messages deleted"),
        })),
        Err(e) => {
            println!("Error deleting all messages: {:?}", e);
            Err(get_failure_status())
        }
    }
}

fn get_failure_status() -> Response<'static> {
    Response::build()
        .status(Status::InternalServerError)
        .finalize()
}
