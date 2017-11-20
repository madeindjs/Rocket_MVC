#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;

mod recipes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


fn main() {
    rocket::ignite()
    	.mount("/", routes![index])
    	.mount("/recipes", routes![recipes::index])
    	.attach(Template::fairing())
    	.launch();
}