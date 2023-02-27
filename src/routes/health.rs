use rocket::*;

#[get("/health")]
pub fn health_route() -> &'static str {
    "Ok"
}
