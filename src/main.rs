#![feature(plugin)]
#![plugin(rocket_codegen, diesel_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

use rocket_contrib::Template;

use diesel::LimitDsl;
use diesel::LoadDsl;


mod recipes_controller;
mod models;
mod database;
mod schema;

use self::diesel::prelude::*;
use self::schema::recipes::dsl::*;

#[get("/")]
fn index() -> Template {
    let connection = database::establish_connection();
    let results = recipes
        .filter(id.eq(1))
        .limit(5)
        .load::<models::Recipe>(&connection)
        .expect("Error loading recipes");

    println!("Displaying {} recipes", results.len());
    for recipe in results {
        println!("{}", recipe.name);
    }

    let map = ();
    Template::render("index", &map)
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/recipes", routes![recipes_controller::index])
        .mount("/recipes", routes![recipes_controller::show])
        .attach(Template::fairing())
        .launch();
}
