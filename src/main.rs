#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod recipes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn recipes() -> &'static str {
    "List of all recipes"
}

fn main() {
    rocket::ignite()
    	.mount("/", routes![index])
    	.mount("/recipes", routes![recipes::index])
    	.launch();
}