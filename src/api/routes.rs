#[path = "./users_handlers.rs"]
mod users_handlers;

use rocket_contrib::json::{Json, JsonValue};
use users_handlers::NewUser;

// routes
#[post("/user", format = "json", data = "<user>")]
pub fn post_user(user: Json<NewUser>) -> JsonValue {
    users_handlers::new_user(user)
}

#[get("/user/<id>")]
pub fn get_user(id: i32) {}
