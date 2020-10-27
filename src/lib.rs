#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod menu_interface;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("Could not find DATABASE_URL environment variable.");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Could not connect to database {}", database_url))
}

pub fn create_hero(conn: &SqliteConnection, name: String, rating: f32) {
    use models::NewHero;
    use schema::heroes;
    use uuid::Uuid;

    let new_hero = NewHero {
        id: Uuid::new_v4().to_string(),
        name: name,
        rating: rating,
    };

    diesel::insert_into(heroes::table)
        .values(&new_hero)
        .execute(conn)
        .expect("Error inserting hero");
}

pub fn get_heroes(conn: &SqliteConnection) {
    use models::Hero;
    use schema::heroes;

    let result = heroes::table
        .load::<Hero>(conn)
        .expect("Erro selecting heroes");
    for hero in result {
        println!("Hero: {}\n", hero.name);
        println!("ID: {}\n", hero.id);
        println!("Rating: {}\n", hero.rating);
    }
}
