use ws::listen;
use std::thread;
use serde_json::error::{Error as SerdeJsonError};
use ws::Message;

use super::types::{SavedMessage};

/// # Open WebSockets listener
/// Handle realtime message communication to connected clients
pub fn run_socket_listener() {
    // Run WebSocket listener on a separate thread to not block the main server thread
    thread::spawn(|| {
        // Listen on an address and call the closure for each connection
        if let Err(error) = listen("127.0.0.1:3012", |out| {
            // The handler needs to take ownership of out, so we use move
            move |msg: Message| {
                println!("Received message via WebSockets");
                // Parse the message
                let result = handle_socket_message(msg);
                match result {
                    Ok(saved_message) => {
                        // Broadcast response if message was valid
                        out.broadcast(format!("{:?}", saved_message))
                    },
                    Err(_) => {
                        // No action if any error
                        Ok(())
                    }
                }
            }
        }) {
            // Log error setting up listener
            println!("Failed to create WebSocket due to {:?}", error);
        }
    });
}

/// # Handle parsing WebSocket messages
/// Parses message to only forward messages if incoming message is valid
fn handle_socket_message(raw_message: Message) -> Result<SavedMessage, &'static str> {
    let maybe_text= raw_message.as_text();
    match maybe_text {
        Ok(text) => {
            let json_result: Result<SavedMessage, SerdeJsonError> = serde_json::from_str(text);
            match json_result {
                Ok(result) => {
                    println!("Parsed WebSocket message: {:?}", result);
                    Ok(result)
                },
                Err(e) => {
                    println!("Error parsing JSON from WebSocket message: {:?}", e);
                    Err("Message parsing failure")
                }
            }
        },
        Err(e) => {
            println!("Could not parse text from WebSocket message: {:?}", e);
            Err("Message parsing failure")
        }
    }
}