
fn main() {
    // Integration tests for the Messenger Rocket server
    println!("This is a Rust program containing the integration tests for the Rocket server!");
}

#[cfg(test)]
mod tests {
    extern crate reqwest;

    use std::{thread, time};

    #[test]
    fn health_check_endpoint() {
        server_status();
        let result = reqwest::get("http://0.0.0.0:8000/rocket");
        let expected = "Hello from Messenger Rocket server";
        assert_eq!(result.unwrap().text().unwrap(), expected);
    }

    // Loop until server is ready
    fn server_status() {
        let mut limit = 0;
        let maximum = 15;
        let mut maybe_response = reqwest::get("http://0.0.0.0:8000/rocket");
        loop {
            match maybe_response {
                Ok(_) => {
                    println!("Rocket is ready - running tests!");
                    break;
                },
                Err(_) => {
                    wait();
                    limit = limit + 1;
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
        println!("Rocket Server not ready yet... retrying in 3 seconds");
        thread::sleep(time::Duration::from_millis(3000));
    }
}