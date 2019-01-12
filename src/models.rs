/// Just a recipe with a name and an id
#[derive(Serialize, Queryable, Clone)]
pub struct Recipe {
    pub id: i32,
    pub name : String
}
