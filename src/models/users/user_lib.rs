use super::super::super::schema::users;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub pin: String,
}
