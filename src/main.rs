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


mod controller;
mod models;
mod database;
mod schema;


fn main() {
    rocket::ignite()
        .mount("/", routes![controller::pages::home])
        .mount("/recipes", routes![controller::recipes::index])
        .mount("/recipes", routes![controller::recipes::show])
        .mount("/recipes", routes![controller::recipes::new])
        .attach(Template::fairing())
        .launch();
}
