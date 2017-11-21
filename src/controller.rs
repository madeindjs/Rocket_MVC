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
        let map = ();

        let connection = database::establish_connection();
        let results = recipes
            .filter(id.eq(1))
            .limit(5)
            .load::<models::Recipe>(&connection)
            .expect("Error loading recipes");

        println!("Displaying {} recipes", results.len());
        for recipe in results {
            println!("{}", recipe.name);
        }


        Template::render("index", &map)
    }

    #[get("/<slug>")]
    pub fn show(slug: String) -> Template {
        // let recipe = models::Recipe{slug: slug};
        let recipe = ();
        Template::render("recipe", recipe)
    }
}

pub mod pages {
    use rocket_contrib::Template;

    #[get("/")]
    pub fn home() -> Template {
        let map = ();
        Template::render("index", &map)
    }
}
