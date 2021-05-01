#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]

// crates
#[macro_use]
extern crate rocket;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::env;

use dotenv::dotenv;
use mysql::PooledConn;
use serde::Serialize;

// imports
use crate::database::conn::init_diesel_conn;

// modules
mod api;
mod database;
mod models;

use api::routes::*;

embed_migrations!();

#[derive(Serialize)]
struct ErrResponse {
    status: u16,
    description: String,
}

fn get_db() -> PooledConn {
    dotenv().ok();
    let db_host: std::string::String = env::var("DB_HOST").expect("DB_HOST must be set");

    let db_user: std::string::String = env::var("DB_USER").expect("DB_USER must be set");
    let db_password: std::string::String =
        env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let dn_port: std::string::String = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_name: std::string::String = env::var("DB_NAME").expect("DB_NAME must be set");

    // database connection
    let pool_conn: PooledConn =
        database::conn::init_database(&db_user, &db_host, &dn_port, &db_password, &db_name);

    return pool_conn;
}

// main
fn main() {
    // DEPRECATED CODE
    /*
    //     let user_query = r"CREATE TABLE IF NOT EXISTS users
    // (
    //     id         int PRIMARY KEY,
    //     created_id DATETIME,
    //     updated_at DATETIME,
    //     deleted_at DATETIME,
    //     first_name VARCHAR(50) NOT NULL,
    //     last_name  VARCHAR(50) NOT NULL,
    //     PIN        VARCHAR(50) NOT NULL
    // )";
    //
    //     let mut pool_conn = get_db();
    //
    //     // create user table
    //     pool_conn.query_drop(user_query).unwrap();
     */
    dotenv::dotenv().ok();

    // Init Diesel db connection
    let mysql_conn = init_diesel_conn();

    // run migrations
    embedded_migrations::run(&mysql_conn);

    rocket::ignite().mount("/v1", routes![post_user]).launch();
}
