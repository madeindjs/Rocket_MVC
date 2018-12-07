#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use rocket_contrib::json::Json;



#[derive(Serialize)]
struct Recipe {
    name : String
}


#[get("/")]
fn index() -> Json<Vec<Recipe>> {
    let recipes : Vec<Recipe> =  vec![Recipe{name: "Recipe A".to_string()}, Recipe{name: "Recipe B".to_string()}];
    Json(recipes)
}

#[get("/a")]
fn show() -> Json<Recipe> {
    Json(Recipe{name: "Recipe A".to_string()})
}


fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/recipes", routes![show])
    .launch();
}
