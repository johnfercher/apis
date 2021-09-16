use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{Pool, ConnectionManager};

pub type PgPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let manager = ConnectionManager::<SqliteConnection>::new("db/sqlite.db");
    Pool::new(manager).expect("Postgres connection pool could not be created")
}