use rocket_contrib::Template;

use recipe;

#[get("/")]
pub fn index() -> Template {
	let map = ();
    Template::render("index", &map)
}

#[get("/<name>")]
pub fn show(name: String) -> Template {
    let recipe = recipe::Recipe{name: name};
    Template::render("recipe", recipe)
}