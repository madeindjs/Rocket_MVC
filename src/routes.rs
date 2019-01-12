use rocket::Rocket;
use controllers;

pub fn build() -> Rocket {
    rocket::ignite()
    .mount("/", routes![
        controllers::pages::home,
    ])
    .mount("/recipes", routes![
        controllers::recipes::index,
        controllers::recipes::show,
    ])
}
