use super::super::super::schema::users;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub first_name: &'a str,
    pub last_name: Option<String>,
    pub pin: &'a str,
}
