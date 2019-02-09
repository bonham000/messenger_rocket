use serde_json::error::Error as SerdeJsonError;
use std::thread;
use ws::listen;
use ws::Message;

use super::types::{SavedMessage, MessageBroadcast, MessageBroadcastType};

/// # Open WebSockets listener
/// Handle realtime message communication to connected clients
pub fn run_socket_listener() {
    // Run WebSocket listener on a separate thread to not block the main server thread
    thread::spawn(|| {
        // Listen on an address and call the closure for each connection
        if let Err(error) = listen("0.0.0.0:3012", |out| {
            // The handler needs to take ownership of out, so we use move
            move |msg: Message| {
                println!("Received message via WebSockets");
                // Parse the message
                let result = handle_socket_message(msg);
                match result {
                    Ok(saved_message) => {
                        // Broadcast response if message was valid
                        let json_broadcast = serde_json::to_string(&saved_message);
                        match json_broadcast {
                            Ok(json) => {
                                out.broadcast(Message::text(json))
                            }
                            Err(_) => {
                                Ok(())
                            }
                        }

                    }
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
fn handle_socket_message(raw_message: Message) -> Result<MessageBroadcast, &'static str> {
    let maybe_text = raw_message.as_text();
    match maybe_text {
        Ok(text) => {
            println!("New WebSocket message received, text: {}", text);
            let json_result: Result<MessageBroadcast, SerdeJsonError> = serde_json::from_str(text);
            match json_result {
                Ok(result) => {
                    println!("Parsed WebSocket message: {:?}", result);
                    Ok(result)
                }
                Err(e) => {
                    println!("Error parsing JSON from WebSocket message: {:?}", e);
                    Err("Message parsing failure")
                }
            }
        }
        Err(e) => {
            println!("Could not parse text from WebSocket message: {:?}", e);
            Err("Message parsing failure")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ws::Message;

    #[test]
    fn test_success_handle_socket_message() {
        let test_input = "{\
                        \"message_type\" : \"NEW\",\
                        \"message\": {\
                              \"id\": 1,\
                              \"message\": \"Hello, from Earth\",\
                              \"author\": \"Seanie X\",\
                              \"uuid\": \"asdf78asd6f89sa6f89a76s8df\"\
                              }\
                          }";
        let test_message = Message::text(test_input);
        let test_result = handle_socket_message(test_message);

        let expected_message: SavedMessage = SavedMessage {
            id: 1,
            message: String::from("Hello, from Earth"),
            author: String::from("Seanie X"),
            uuid: String::from("asdf78asd6f89sa6f89a76s8df"),
        };
        let expected_result: MessageBroadcast = MessageBroadcast {
            message_type: MessageBroadcastType::NEW,
            message: expected_message,
        };
        assert_eq!(test_result.unwrap(), expected_result);
    }

    #[test]
    fn test_failure_handle_socket_message() {
        let mut test_input = "{\
                              \"id\": 1,\
                              message: \"Hello, from Earth\",\
                              \"author\": \"Seanie X\",\
                              \"uuid\": \"asdf78asd6f89sa6f89a76s8df\"\
                              }";
        let test_message = Message::text(test_input);
        let test_result = handle_socket_message(test_message);
        assert!(test_result.is_err(), "Message parsing failure");

        test_input = "{{ - && }}}";
        let test_message = Message::text(test_input);
        let test_result = handle_socket_message(test_message);
        assert!(test_result.is_err(), "Message parsing failure");
    }
}
