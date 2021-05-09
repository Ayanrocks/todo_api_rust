#[path = "./user_lib.rs"]
mod user_lib;
use crate::models::schema::users;

use crate::models::users::users::user_lib::NewUser;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{sql_query, MysqlConnection};

#[derive(Queryable, QueryableByName)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
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
    use super::super::schema::users;
    // use super::super::schema::users::dsl::*;

    let new_user = NewUser {
        user_name,
        first_name,
        last_name: Some(last_name.parse().unwrap()),
        pin: hashed_pin,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new users")
}

pub fn check_user_exists(conn: &MysqlConnection, username: &'a str) -> bool {
    use super::super::schema::users::dsl::*;

    let results = diesel::dsl::sql_query(
        "SELECT * FROM users u WHERE u.deleted_at is NULL AND u.user_name = ?",
    )
    .bind::<&str, _>(username)
    .get_result(conn)
    .expect("error loadi ng users");

    if results.len() == 0 {
        return true;
    }

    return false;
}
