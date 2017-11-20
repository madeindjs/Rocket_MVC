#[get("/")]
pub fn index() -> &'static str {
    "list of recipes"
}
