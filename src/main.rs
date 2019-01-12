#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate dotenv;
extern crate serde_json;

// use diesel::prelude::*;

use diesel::RunQueryDsl;
use rocket_contrib::json::Json;

mod models;
mod schema;
mod database;


// use schema::recipes;
use models::Recipe;

// #[derive(Insertable)]
// #[table_name="recipes"]
// pub struct NewRecipe {
//     pub name: String,
// }


#[get("/")]
fn index() -> Json<Vec<Recipe>> {
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
fn show(id: i32) -> Json<Recipe> {
    Json(Recipe{id: id, name: format!("Recipe {}", id)})
}


fn main() {
    rocket::ignite()
    .mount("/recipes", routes![index, show])
    .launch();
}
