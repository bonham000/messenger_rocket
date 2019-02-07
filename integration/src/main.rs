
fn main() {
    // Integration tests for the Messenger Rocket server
}

#[cfg(test)]
mod tests {
    extern crate hyper;
    extern crate reqwest;

    use std::{thread, time};

    #[test]
    fn health_check_endpoint() {
        thread::sleep(time::Duration::from_millis(5000));
        let body = reqwest::get("http://0.0.0.0:8000/rocket").unwrap()
            .text().unwrap();

        let expected = "Hello from Messenger Rocket server";
        assert_eq!(body, expected);
    }
}