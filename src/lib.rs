#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Could not find DATABASE_URL environment variable.");
    SqliteConnection::establish(&database_url).expect(&format!("Could not connect to database {}", database_url))
}

