#[path = "./user_lib.rs"]
mod user_lib;

use crate::models::users::users::user_lib::NewUser;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::MysqlConnection;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
    pub user_name: String,
    pub first_name: String,
    pub last_name: Option<String>,
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
    use super::super::schema::users::dsl::*;
    // use super::super::schema::users::dsl::*;

    let new_user = NewUser {
        user_name,
        first_name,
        last_name: Some(last_name.parse().unwrap()),
        pin: hashed_pin.to_string(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .default_values()
        .execute(conn)
        .expect("Error saving new users")
}

pub fn check_user_exists(conn: &MysqlConnection, username: &'a str) -> bool {
    use super::super::schema::users::dsl::*;

    let result = users
        .filter(user_name.eq(username))
        .limit(5)
        .load::<User>(conn)
        .expect("error loading users");

    if result.len() == 0 {
        return true;
    }

    return false;
}
