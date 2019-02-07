#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

extern crate dotenv;
use dotenv::dotenv;

mod controllers;
mod postgres;
mod repository;
mod routes;
mod schema;
mod service;
mod socket;
mod types;

fn main() {
    // Load environment variables
    dotenv().ok();

    // Run the WebSockets listener
    socket::run_socket_listener();

    // Setup Rocket and fire!
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    routes::build_server()
}

//#[cfg(test)]
//mod test {
//    use super::rocket;
//    use rocket::local::Client;
//    use rocket::http::Status;
//
//    #[test]
//    fn hello_world() {
//        let client = Client::new(rocket()).expect("valid rocket instance");
//        let mut response = client.get("/messages").dispatch();
//        assert_eq!(response.status(), Status::Ok);
//        assert_eq!(response.body_string(), Some("[]".into()));
//    }
//}
