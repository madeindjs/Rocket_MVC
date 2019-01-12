#![feature(proc_macro_hygiene, decl_macro)]
/// This file is used to load all modules needed to run units tests
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

mod controllers;
mod database;
mod forms;
pub mod models;
pub mod routes;
mod schema;
