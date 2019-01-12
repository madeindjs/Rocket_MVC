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


mod models;
mod schema;
mod database;
mod controllers;
mod routes;

// use schema::recipes;

// #[derive(Insertable)]
// #[table_name="recipes"]
// pub struct NewRecipe {
//     pub name: String,
// }




fn main() {
    routes::build().launch();
}
