#[derive(Queryable)]
pub struct Hero {
    pub id: String,
    pub name: String,
    pub rating: f32,
}

use super::schema::heroes;

#[derive(Insertable)]
#[table_name = "heroes"]
pub struct NewHero {
    pub id: String,
    pub name: String,
    pub rating: f32,
}
