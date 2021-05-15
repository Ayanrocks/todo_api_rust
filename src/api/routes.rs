#[path = "./users_handlers.rs"]
mod users_handlers;

use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use rocket_contrib::json::{Json, JsonValue};
use users_handlers::NewUser;

#[derive(Debug)]
pub struct APIResponse {
    pub status: Status,
    pub description: JsonValue,
}

impl<'r> Responder<'r> for APIResponse {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        Response::build_from(self.description.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

// routes
#[post("/user", format = "json", data = "<user>")]
pub fn post_user(user: Json<NewUser>) -> APIResponse {
    users_handlers::new_user(user)
}

#[get("/user/<id>")]
pub fn get_user(id: i32) {}
