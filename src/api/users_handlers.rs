use rocket::http::Status;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser<'a> {
    user_name: &'a str,
    first_name: &'a str,
    last_name: &'a str,
    pin: &'a str,
}

pub fn new_user(user: Json<NewUser>) -> JsonValue {
    // validation
    if user.last_name == "" || user.first_name == "" {
        Status::BadRequest;
        return json!({
            "Error": "first_name or last_name is missing",
        });
    }

    json!({
        "hello": "f",
    })
}
