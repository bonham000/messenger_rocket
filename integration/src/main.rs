#[macro_use]
extern crate serde_derive;
extern crate reqwest;

fn main() {
    // Integration tests for the Messenger Rocket server
    println!("This is a Rust program containing the integration tests for the Rocket server!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::{thread, time};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct SavedMessage {
        pub id: i32,
        pub message: String,
        pub author: String,
        pub uuid: String,
    }

    #[test]
    fn health_check_endpoint() {
        wait_for_server();
        let result = reqwest::get("http://0.0.0.0:8000/rocket");
        let expected = "Hello from Messenger Rocket server";
        assert_eq!(result.unwrap().text().unwrap(), expected);
    }

    #[test]
    fn get_messages_is_empty_array_initially() {
        wait_for_server();
        let result = reqwest::get("http://0.0.0.0:8000/messages");
        let expected = "[]";
        assert_eq!(result.unwrap().text().unwrap(), expected);
    }

    #[test]
    fn post_message_adds_messages() {
        wait_for_server();
        let result = reqwest::get("http://0.0.0.0:8000/messages");
        let expected = "[]";
        assert_eq!(result.unwrap().text().unwrap(), expected);

        let message = String::from("Hello, from Earth");
        let author = String::from("Seanie X");

        let mut test_message = HashMap::new();
        test_message.insert("message", &message);
        test_message.insert("author", &author);

        let client = reqwest::Client::new();
        let response = client
            .post("http://0.0.0.0:8000/message")
            .json(&test_message)
            .send();

        let post_message_result: SavedMessage =
            serde_json::from_str(&response.unwrap().text().unwrap()).unwrap();

        assert_eq!(post_message_result.message, message);
        assert_eq!(post_message_result.author, author);
    }

    // Loop until server is ready
    fn wait_for_server() {
        let mut limit = 0;
        let maximum = 30;
        let mut maybe_response = reqwest::get("http://0.0.0.0:8000/rocket");
        loop {
            match maybe_response {
                Ok(_) => {
                    println!("Rocket is ready - running tests!");
                    break;
                }
                Err(_) => {
                    wait();
                    limit = limit + 1;
                    if limit % 5 == 0 {
                        println!("Rocket Server not ready yet... waiting...");
                    }

                    if limit > maximum {
                        panic!("Maximum retries reached - aborting tests!");
                    } else {
                        maybe_response = reqwest::get("http://0.0.0.0:8000/rocket");
                    }
                }
            }
        }
    }

    fn wait() {
        thread::sleep(time::Duration::from_millis(5000));
    }
}
