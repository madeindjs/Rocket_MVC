// use models;

pub mod recipes {
    use rocket_contrib::Template;

    #[get("/")]
    pub fn index() -> Template {
        let map = ();
        Template::render("index", &map)
    }

    #[get("/<name>")]
    pub fn show(name: String) -> Template {
        // let recipe = models::Recipe{name: name};
        let recipe = ();
        Template::render("recipe", recipe)
    }
}
