// use models;

pub mod recipes {
    use rocket_contrib::Template;
    use diesel::prelude::*;
    use schema::recipes::dsl::*;
    use diesel::LimitDsl;
    use diesel::LoadDsl;

    use models;
    use database;

    #[get("/")]
    pub fn index() -> Template {
        let connection = database::establish_connection();
        let results = recipes
            .limit(20)
            .load::<models::Recipe>(&connection)
            .expect("Error loading recipes");

        Template::render("recipes/index", &results)
    }

    #[get("/<recipe_id>")]
    pub fn show(recipe_id: i32) -> Template {
        let connection = database::establish_connection();
        let results = recipes
            .filter(id.eq(recipe_id))
            .limit(1)
            .load::<models::Recipe>(&connection)
            .expect("Error loading recipes");
        Template::render("recipes/show", results.first())
    }

    #[get("/new")]
    pub fn new() -> Template {
        Template::render("recipes/new", &())
    }
}

pub mod pages {
    use rocket_contrib::Template;

    #[get("/")]
    pub fn home() -> Template {
        let map = ();
        Template::render("pages/home", &map)
    }
}
