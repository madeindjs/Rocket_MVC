#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/")]
fn recipes() -> &'static str {
    "List of all recipes"
}

fn main() {
    rocket::ignite()
    	.mount("/", routes![index])
    	.mount("recipes", routes![recipes])
    	.launch();
}