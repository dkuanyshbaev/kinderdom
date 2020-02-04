pub mod articles;
pub mod events;
pub mod profiles;
pub mod projects;

use crate::Db;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

#[get("/")]
pub fn main(_conn: Db) -> Template {
    let context = TemplateContext {
        name: "main admin page".to_string(),
        items: vec!["one", "two"],
    };
    Template::render("admin/main", &context)
}

#[get("/login")]
pub fn login(_conn: Db) -> Template {
    let context = TemplateContext {
        name: "login page".to_string(),
        items: vec!["one", "two"],
    };
    Template::render("admin/login", &context)
}
