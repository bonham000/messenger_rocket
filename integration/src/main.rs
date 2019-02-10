#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;

fn main() {
    // Integration tests for the Messenger Rocket server
    println!("This is a Rust program containing the integration tests for the Rocket server!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::vec::Vec;
    use std::{thread, time};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct SavedMessage {
        pub id: i32,
        pub message: String,
        pub author: String,
        pub uuid: String,
    }

    #[test]
    fn test_messages_rest_api() {
        let client = reqwest::Client::new();

        wait_for_server();

        let result = reqwest::get(&get_request_path("/rocket"));
        let expected = "Hello from Messenger Rocket server";
        assert_eq!(result.unwrap().text().unwrap(), expected);

        assert_message_history_is_empty();

        let message = String::from("Hello, from Earth");
        let author = String::from("Seanie X");

        let test_message = get_new_message(&author, &message);

        let response = client
            .post(&get_request_path("/message"))
            .json(&test_message)
            .send();

        let post_message_result: SavedMessage =
            serde_json::from_str(&response.unwrap().text().unwrap()).unwrap();

        // Assert a new post message returns a matching message object
        assert_eq!(post_message_result.message, message);
        assert_eq!(post_message_result.author, author);

        let message_list = fetch_messages();

        // Assert the new message is returned by GET /messages
        assert_eq!(
            message_exists_in_list(message_list, &post_message_result),
            true
        );

        let edit_message_text = String::from("Hello, from Venus");
        let edited_message: SavedMessage = SavedMessage {
            id: post_message_result.id,
            uuid: post_message_result.uuid,
            author: post_message_result.author,
            message: edit_message_text.clone(),
        };

        let edit_response = client
            .put(&get_request_path("/message"))
            .json(&edited_message)
            .send();

        let edit_message_result: SavedMessage =
            serde_json::from_str(&edit_response.unwrap().text().unwrap()).unwrap();

        // Assert the edit message response returns the matching message object
        assert_eq!(edit_message_result.message, edit_message_text);
        assert_eq!(edit_message_result.author, author);

        let edit_message_list = fetch_messages();

        // Assert the edited message is now returned by GET /messages
        assert!(
            message_exists_in_list(edit_message_list, &edit_message_result),
            true
        );

        // Verify the message can be delete
        delete_by_message_id(edit_message_result.id);

        assert_message_history_is_empty();

        // Verify paginated fetching works
        save_multiple_messages();

        let result = fetch_messages();
        assert_eq!(result.len(), 3);

        let result = fetch_messages_paginated(1);
        assert_eq!(result.len(), 1);

        let result = fetch_messages_paginated(2);
        assert_eq!(result.len(), 2);

        let result = fetch_messages();

        for m in result.iter() {
            delete_by_message_id(m.id);
        }

        assert_message_history_is_empty();
    }

    fn delete_by_message_id(id: i32) {
        let client = reqwest::Client::new();

        let id_str = id.to_string();
        let delete_path = ["/message/", &id_str].concat();
        let result = client.delete(&get_request_path(&delete_path)).send();

        assert_eq!(
            &result.unwrap().text().unwrap(),
            "{\"status\":\"Message deleted!\"}"
        );
    }

    fn save_multiple_messages() {
        let client = reqwest::Client::new();

        let message_list: Vec<HashMap<&str, &str>> = vec![
            get_new_message("Seanie X", "Hello!!!"),
            get_new_message("Seanie X", "Goodbye!!!"),
            get_new_message("Seanie X", "Blah blah blah!!!"),
        ];

        let mut result;

        for m in message_list.iter() {
            result = client.post(&get_request_path("/message")).json(&m).send();

            let post_message_result: SavedMessage =
                serde_json::from_str(&result.unwrap().text().unwrap()).unwrap();

            let text = (*m.get("message").unwrap()).to_string();
            assert_eq!(post_message_result.message, text);
        }
    }

    fn get_new_message<'a>(author: &'a str, message: &'a str) -> HashMap<&'a str, &'a str> {
        let mut new_message = HashMap::new();
        new_message.insert("message", message);
        new_message.insert("author", author);
        new_message
    }

    fn fetch_messages() -> Vec<SavedMessage> {
        let result = reqwest::get(&get_request_path("/messages"));
        let message_list = serde_json::from_str(&result.unwrap().text().unwrap()).unwrap();
        message_list
    }

    fn fetch_messages_paginated(limit: i64) -> Vec<SavedMessage> {
        let limit_str = limit.to_string();
        let path = ["/messages?limit=", &limit_str].concat();

        let result = reqwest::get(&get_request_path(&path));
        let message_list = serde_json::from_str(&result.unwrap().text().unwrap()).unwrap();
        message_list
    }

    fn message_exists_in_list(
        message_list: Vec<SavedMessage>,
        target_message: &SavedMessage,
    ) -> bool {
        let target_author = &target_message.author;
        let target_message = &target_message.message;

        for m in message_list.iter() {
            if m.author == *target_author && m.message == *target_message {
                return true;
            }
        }

        false
    }

    fn assert_message_history_is_empty() {
        let result = reqwest::get(&get_request_path("/messages"));
        let expected = "[]";
        assert_eq!(result.unwrap().text().unwrap(), expected);
    }

    fn get_request_path(path: &str) -> String {
        ["http://0.0.0.0:8000", path].concat()
    }

    // Loop until server is ready, or timeout
    fn wait_for_server() {
        let mut limit = 0;
        let maximum = 15;
        let mut maybe_response = reqwest::get(&get_request_path("/rocket"));
        loop {
            match maybe_response {
                Ok(_) => {
                    let client = reqwest::Client::new();
                    let result = client.delete(&get_request_path("/admin/delete")).send();
                    assert_eq!(
                        &result.unwrap().text().unwrap(),
                        "{\"status\":\"All messages deleted\"}"
                    );
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
                        maybe_response = reqwest::get(&get_request_path("/messages"));
                    }
                }
            }
        }
    }

    fn wait() {
        thread::sleep(time::Duration::from_millis(3000));
    }
}
