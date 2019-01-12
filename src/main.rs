#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// use diesel::prelude::*;


use rocket_contrib::json::Json;

mod models;
mod schema;

use schema::recipes;
use models::Recipe;

#[derive(Insertable)]
#[table_name="recipes"]
pub struct NewRecipe {
    pub name: String,
}


#[get("/")]
fn index() -> Json<Vec<Recipe>> {
    let recipes : Vec<Recipe> =  vec![Recipe{name: "Recipe A".to_string()}, Recipe{name: "Recipe B".to_string()}];
    Json(recipes)
}

#[get("/<id>")]
fn show(id: usize) -> Json<Recipe> {
    Json(Recipe{name: format!("Recipe {}", id)})
}


fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/recipes", routes![show])
    .launch();
}
