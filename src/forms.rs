use super::schema::recipes;

#[derive(Insertable)]
#[table_name = "recipes"]
#[derive(Debug, FromForm)]
pub struct Recipe {
    pub name: String,
}
