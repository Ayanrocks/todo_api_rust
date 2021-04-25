#[path = "./users_handlers.rs"]
mod users_handlers;

// routes
#[post("/user")]
pub fn post_user() -> &'static str {
    users_handlers::new_user()
}

#[get("/user/<id>")]
pub fn get_user() {}
