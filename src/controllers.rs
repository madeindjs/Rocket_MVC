/// Contains REST routes for recipes
pub mod recipes {

    use diesel::RunQueryDsl;
    use rocket::request::Form;
    use rocket::http::Status;
    use rocket_contrib::json::Json;
    use schema::recipes::dsl::*;

    use models;
    use database;
    use forms;

    /// List all recipes into a beautifull JSON
    #[get("/")]
    pub fn index() -> Json<Vec<models::Recipe>> {
        let connection = database::establish_connection();
        let results = recipes
            .load::<models::Recipe>(&connection)
            .expect("Error loading recipes");

        Json(results)
    }

    /// Get the given recipe from id and render it into a beautifull JSON
    #[get("/<recipe_id>")]
    pub fn show(recipe_id: i32) -> Json<models::Recipe> {
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;

        let connection = database::establish_connection();
        let results = recipes
            .filter(id.eq(&recipe_id))
            .limit(1)
            .load::<models::Recipe>(&connection)
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

    #[post("/", data = "<form_data>")]
    pub fn create(form_data: Form<forms::Recipe>) -> Status {
        use diesel;
        use schema;
        let connection = database::establish_connection();
        let recipe = models::NewRecipe {
            id: None,
            name: form_data.name.to_string(),
        };

        let query = diesel::insert_into(schema::recipes::table).values(&recipe);

        match query.execute(&connection) {
            Ok(_) => Status::Created,
            Err(error) =>{
                println!("Cannot create the recipe: {:?}", error);
                Status::BadRequest
            },
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
