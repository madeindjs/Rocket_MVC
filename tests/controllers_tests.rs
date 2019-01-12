extern crate rocket;
extern crate rocket_mvc;

use rocket::local::Client;
use rocket_mvc::routes;

fn get_client() -> Client {
    Client::new(routes::build()).expect("valid rocket instance")
}

#[cfg(test)]
mod pages {
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn home() {
        let client: Client = super::get_client();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}

#[cfg(test)]
mod recipes {
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn index() {
        let client: Client = super::get_client();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}
