#[path = "./user_lib.rs"]
mod user_lib;

use crate::models::users::users::user_lib::NewUser;
use diesel::prelude::*;
use diesel::MysqlConnection;
use mysql::chrono::{DateTime, Utc};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: DateTime<Utc>,
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub pin: String,
}

/// create_user creates a user for returns it back
pub fn create_user<'a>(
    conn: &MysqlConnection,
    user_name: &'a str,
    first_name: &'a str,
    last_name: &'a str,
    hashed_pin: &'a str,
) -> usize {
    use super::super::schema::users;
    // use super::super::schema::users::dsl::*;

    let new_user = NewUser {
        user_name: user_name.to_string(),
        first_name: first_name.to_string(),
        last_name: Some(last_name.parse().unwrap()),
        pin: hashed_pin.to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new users")
}
