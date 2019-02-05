use ws::listen;
use std::thread;
use ws::Message;

use super::service;

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
                let result = service::handle_socket_message(msg);
                match result {
                    Ok(saved_message) => {
                        // Broadcast response if message was valid
                        out.broadcast(format!("{:?}", saved_message))
                    },
                    Err(_) => {
                        // No action if any error
                        println!("Error sending message...");
                        Ok(())
                    }
                }
            }
        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to {:?}", error);
        }
    });
}