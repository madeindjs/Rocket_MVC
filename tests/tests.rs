#[macro_use]
extern crate rocket_mvc;
extern crate rocket;

#[cfg(test)]
mod test {
    use rocket_mvc::routes;
    use rocket::local::Client;
    use rocket::http::Status;

    fn get_client() -> Client {
        Client::new(routes::build()).expect("valid rocket instance")
    }

    #[test]
    fn pages_home() {
         let client : Client = get_client();
         let response = client.get("/").dispatch();

         assert_eq!(response.status(), Status::Ok);
    }
}
