use rocket_contrib::json::Json;

#[post("/", format="json", data = "<message>")]
pub fn post_message(message: Json<Message>) -> Json<Status> {
    println!("Message received: {:?}", message);
    // Handle saving the message now
    Json(Status {
        status: String::from("OK"),
    })
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