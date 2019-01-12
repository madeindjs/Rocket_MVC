use rocket::Rocket;
use controllers;

/// Mount all rockets routes
pub fn build() -> Rocket {
    rocket::ignite()
    .mount("/", routes![
        controllers::pages::home,
    ])
    .mount("/recipes", routes![
        controllers::recipes::index,
        controllers::recipes::show,
        controllers::recipes::create,
    ])
}
