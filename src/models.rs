use super::schema::recipes;

/// Just a recipe with a name and an id
#[derive(Serialize, Queryable, Clone)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
}

// Used to insert a new recipe into the database
#[table_name = "recipes"]
#[derive(Insertable)]
pub struct NewRecipe {
    pub id: Option<i32>,
    pub name: String,
}
