/// Contains REST routes for recipes
pub mod recipes {

    use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
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
        Json(get_recipe(&recipe_id))
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

    #[put("/<recipe_id>", data = "<form_data>")]
    pub fn update(recipe_id: i32, form_data: Form<forms::Recipe>) -> Status {
        let connection = database::establish_connection();

        let result = diesel::update(recipes.find(recipe_id))
            .set(name.eq(form_data.name.to_string()))
            .execute(&connection);

        match result {
            Ok(_) => Status::Accepted,
            Err(error) => {
                println!("Cannot update the recipe: {:?}", error);
                Status::BadRequest
            },
        }
    }

    #[delete("/<recipe_id>")]
    pub fn delete(recipe_id: i32) -> Status {
        let connection = database::establish_connection();

        match diesel::delete(recipes.find(recipe_id)).execute(&connection) {
            Ok(_) => Status::Accepted,
            Err(error) => {
                println!("Cannot delete the recipe: {:?}", error);
                Status::BadRequest
            },
        }
    }

    /// find the recipe and panic if not found
    fn get_recipe(recipe_id: &i32) -> models::Recipe {
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;

        let connection = database::establish_connection();
        let results = recipes
            .filter(id.eq(recipe_id))
            .limit(1)
            .load::<models::Recipe>(&connection)
            .expect("Error loading recipes");

        match results.first() {
            Some(recipe) => {
                recipe.clone()
            }
            None => {
                panic!("Recipe not found")
            }
        }
    }

}

/// Somes pages who not linked with any model
pub mod pages {

    #[get("/")]
    pub fn home() -> String {
        "Hello world".to_string()
    }
}
