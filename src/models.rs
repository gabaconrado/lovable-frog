#[derive(Queryable)]
pub struct Hero {
    pub id: String,
    pub name: String,
    pub rating: f32,
}
