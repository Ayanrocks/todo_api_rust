#![feature(proc_macro_hygiene, decl_macro)]

// crates
#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate serde;
extern crate serde_json;

// imports
use chrono::Utc;
use mysql::prelude::*;
use mysql::PooledConn;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

// modules
pub mod database;

// structs
#[derive(Deserialize)]
struct User {
    first_name: String,
    last_name: String,
    pin: String,
}

#[derive(Serialize)]
struct ErrResponse {
    status: u16,
    description: String,
}

// routes
#[post("/user", format = "json", data = "<user>")]
fn new_user(user: Json<User>) -> Status {
    println!(
        "User details: {}, {}. {}",
        user.first_name, user.last_name, user.pin
    );

    // field validation
    if user.first_name == "" || user.last_name == "" || user.pin == "" {
        return Status::BadRequest;
    }

    // get db instance
    let mut pool_conn = get_db();

    // store it in db
    let insert_user_query = r"
        INSERT INTO users (id, created_id, updated_at, first_name, last_name, PIN) VALUES (?, ?, ?, ?, ?, ?)
    ";

    let now_time = mysql::chrono::DateTime::<Utc>::from(Utc::now()).to_string();

    pool_conn
        .exec_drop(
            insert_user_query,
            (
                1,
                &now_time,
                &now_time,
                &user.first_name,
                &user.last_name,
                &user.pin,
            ),
        )
        .unwrap();

    Status::Accepted
}

fn get_db() -> PooledConn {
    const DB_HOST: &str = "localhost";

    const DB_USER: &str = "root";
    const DB_PASSWORD: &str = "1234";
    const DB_PORT: &str = "3306";
    const DB_NAME: &str = "todo_rust";

    // database connection
    let pool_conn: PooledConn =
        database::conn::init_database(DB_USER, DB_HOST, DB_PORT, DB_PASSWORD, DB_NAME);

    return pool_conn;
}

// main
fn main() {
    let user_query = r"CREATE TABLE IF NOT EXISTS users
(
    id         int PRIMARY KEY,
    created_id DATETIME,
    updated_at DATETIME,
    deleted_at DATETIME,
    first_name VARCHAR(50) NOT NULL,
    last_name  VARCHAR(50) NOT NULL,
    PIN        VARCHAR(50) NOT NULL
)";

    let mut pool_conn = get_db();

    // create user table
    pool_conn.query_drop(user_query).unwrap();

    rocket::ignite().mount("/v1", routes![new_user]).launch();
}
