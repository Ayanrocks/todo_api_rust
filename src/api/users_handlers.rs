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
    if user.user_name.len() < 5 {
        return APIResponse {
            status: Status::BadRequest,
            description: json!({
                "Status": "user name needs to be atleast 5 characters"
            }),
        };
    }

    if user.first_name.len() < 2 {
        return APIResponse {
            status: Status::BadRequest,
            description: json!({
                "Status": "first name needs to be atleast 2 characters"
            }),
        };
    }

    if user.last_name.unwrap_or("") != "" && user.last_name.unwrap_or("").len() < 2 {
        return APIResponse {
            status: Status::BadRequest,
            description: json!({
                "Status": "last name needs to be atleast 2 characters"
            }),
        };
    }

    if user.pin.len() == 4 {
        return APIResponse {
            status: Status::BadRequest,
            description: json!({
                "Status": "pin needs to be 4 characters"
            }),
        };
    }

    // check if the user name exists
    // todo if same name doesnt exists then add user to db

    return APIResponse {
        status: Status::Created,
        description: json!({
            "Status": "user created successfully"
        }),
    };

    // return;
}
