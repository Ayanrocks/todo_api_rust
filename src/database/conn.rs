use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use mysql::prelude::*;
use mysql::*;
use std::env;

/// inits database and returns a pool connection
pub fn init_database(
    db_user: &str,
    db_host: &str,
    db_port: &str,
    db_password: &str,
    db_name: &str,
) -> PooledConn {
    let conn_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name,
    );

    let pool = match Pool::new(conn_url) {
        Ok(pool) => pool,
        _ => {
            panic!("Something went wrong");
        }
    };

    pool.get_conn().unwrap()
}

/// inits diesel connection to the DB
pub fn init_diesel_conn() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
