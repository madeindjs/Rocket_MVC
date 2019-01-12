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

mod models;
mod forms;
mod schema;
mod database;
mod controllers;
mod routes;

fn main() {
    routes::build().launch();
}
