use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Context {}

#[get("/")]
pub fn index() -> Template {
    let context = Context {};

    Template::render("index", context)
}
