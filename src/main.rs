extern crate diesel;
extern crate lovable_frog;

use self::diesel::prelude::*;

fn main() {
    use lovable_frog::schema::heroes;
    let connection = lovable_frog::establish_connection();
    let results = heroes::table.find("123").execute(&connection);
    println!("Hello, world!");
}
