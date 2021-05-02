use rocket::http::Status;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use super::super::routes::APIResponse;

#[derive(Deserialize)]
pub struct NewUser<'a> {
    user_name: &'a str,
    first_name: &'a str,
    last_name: Option<&'a str>,
    pin: &'a str,
}

pub fn new_user(user: Json<NewUser>) -> APIResponse {
    // validation
    if user.first_name.len() < 2 {
        return APIResponse {
            status: Status::BadRequest,
            description: json!({
                "Status": "first name need to be atleast 2 characters"
            }),
        };
    }

    if user.last_name.unwrap_or("") != "" && user.last_name.unwrap_or("").len() < 2 {
        return APIResponse {
            status: Status::BadRequest,
            description: json!({
                "Status": "last name need to be atleast 2 characters"
            }),
        };
    }

    return APIResponse {
        status: Status::Created,
        description: json!({
            "Status": "user created successfully"
        }),
    };

    // return;
}
