extern crate diesel;

use lovable_frog;
use lovable_frog::diesel::prelude::*;

fn main() {
    use lovable_frog::schema::heroes;
    use lovable_frog::models::Hero;

    let connection = lovable_frog::establish_connection();
    let results = heroes::table.select(heroes::id).limit(5).load::<Hero>(&connection).expect("Error");
    println!("Hello, world!");
}
