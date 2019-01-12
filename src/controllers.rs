
pub mod recipes {

    use diesel::RunQueryDsl;
    use rocket_contrib::json::Json;
    use models::Recipe;
    use database;

    #[get("/")]

    pub fn index() -> Json<Vec<Recipe>> {
        use schema::recipes::dsl::*;

        let connection = database::establish_connection();
        let results = recipes
            // .limit(20)
            .load::<Recipe>(&connection)
            .expect("Error loading recipes");


        // let recipes : Vec<Recipe> =  vec![Recipe{name: "Recipe A".to_string()}, Recipe{name: "Recipe B".to_string()}];
        Json(results)
    }

    #[get("/<id>")]
    pub fn show(id: i32) -> Json<Recipe> {
        Json(Recipe{id: id, name: format!("Recipe {}", id)})
    }

}
