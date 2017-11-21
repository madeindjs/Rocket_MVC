#[derive(Deserialize, Queryable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
}
