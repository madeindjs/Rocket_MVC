/// Contains REST routes for recipes
pub mod recipes {

    use diesel::RunQueryDsl;
    use rocket_contrib::json::Json;
    use schema::recipes::dsl::*;

    use models::Recipe;
    use database;

    /// List all recipes into a beautifull JSON
    #[get("/")]
    pub fn index() -> Json<Vec<Recipe>> {
        let connection = database::establish_connection();
        let results = recipes
            .load::<Recipe>(&connection)
            .expect("Error loading recipes");

        Json(results)
    }

    /// Get the given recipe from id and render it into a beautifull JSON
    #[get("/<recipe_id>")]
    pub fn show(recipe_id: i32) -> Json<Recipe> {
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;

        let connection = database::establish_connection();
        let results = recipes
            .filter(id.eq(&recipe_id))
            .limit(1)
            .load::<Recipe>(&connection)
            .expect("Error loading recipes");

        match results.first() {
            Some(recipe) => {
                Json(recipe.clone())
            }
            None => {
                panic!("Recipe not found")
            }
        }
    }

}

/// Somes pages who not linked with any model
pub mod pages {

    ///
    #[get("/")]
    pub fn home() -> String {
        "Hello world".to_string()
    }
}
