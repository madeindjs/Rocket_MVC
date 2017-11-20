use rocket_contrib::Template;

#[get("/")]
pub fn index() -> Template {
	let map = ();
    Template::render("index", &map)
}
