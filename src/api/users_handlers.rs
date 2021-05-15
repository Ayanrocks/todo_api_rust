use rocket::http::Status;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use super::super::routes::APIResponse;
use crate::database::conn::init_diesel_conn;
use crate::models::users::users::{check_user_exists, create_user};

#[derive(Deserialize)]
pub struct NewUser<'a> {
    user_name: &'a str,
    first_name: &'a str,
    last_name: Option<&'a str>,
    pin: &'a str,
}

pub fn new_user(user: Json<NewUser>) -> APIResponse {
    let last_name = user.last_name.unwrap();

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

    if last_name != "" && last_name.len() < 2 {
        return APIResponse {
            status: Status::BadRequest,
            description: json!({
                "Status": "last name needs to be atleast 2 characters"
            }),
        };
    }

    if user.pin.len() != 4 {
        return APIResponse {
            status: Status::BadRequest,
            description: json!({
                "Status": "pin needs to be 4 characters"
            }),
        };
    }

    // init db connection
    let conn = init_diesel_conn();

    // check if the user name exists
    let if_exists = check_user_exists(&conn, user.user_name);

    if !if_exists {
        return APIResponse {
            status: Status::BadRequest,
            description: json!({
                "Status": "duplicate user"
            }),
        };
    }

    // create a new user now
    let rows_inserted = create_user(&conn, user.user_name, user.first_name, last_name, user.pin);

    println!("{}", rows_inserted);

    return APIResponse {
        status: Status::Created,
        description: json!({
            "Status": "user created successfully"
        }),
    };

    // return;
}
