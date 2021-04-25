use diesel::prelude::*;
use diesel::{MysqlConnection, RunQueryDsl};
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

pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub pin: &'a str,
}

/// create_user creates a user for returns it back
pub fn create_user<'a>(
    conn: &MysqlConnection,
    user_name: &'a str,
    first_name: &'a str,
    last_name: &'a str,
    hashed_pin: &'a str,
) -> User {
    use crate::models::schema::users;

    let new_user = NewUser {
        user_name,
        first_name,
        last_name,
        pin: hashed_pin,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new users")
}
