#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use rocket_contrib::Template;

mod recipes;
mod recipe;


#[get("/")]
fn index() -> Template {
    let map = ();
    Template::render("index", &map)
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/recipes", routes![recipes::index])
        .mount("/recipes", routes![recipes::show])
        .attach(Template::fairing())
        .launch();
}
