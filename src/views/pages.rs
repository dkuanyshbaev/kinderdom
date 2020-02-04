use crate::Db;
use rocket::Request;
// use rocket::State;
use crate::models::profile::Profile;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

#[get("/")]
// pub fn index(config: State<Config>) -> Template {
pub fn index(connection: Db) -> Template {
    // let secret = &config.secret;
    // println!("{}", secret);
    // -----------------------------------------------------------

    let ps = Profile::all(&connection);
    for p in ps {
        println!("profile: {:?}", p);
    }

    // -----------------------------------------------------------

    let name = "Denis".to_string();
    let context = TemplateContext {
        name,
        items: vec!["One", "Two", "Three"],
    };
    Template::render("index", &context)
}

#[get("/articles")]
pub fn articles() -> &'static str {
    "articles"
}

#[get("/profiles")]
pub fn profiles() -> &'static str {
    "profiles"
}

#[get("/projects")]
pub fn projects() -> &'static str {
    "projects"
}

#[get("/events")]
pub fn events() -> &'static str {
    "events"
}

#[get("/about")]
pub fn about() -> &'static str {
    "about"
}

#[get("/docs")]
pub fn docs() -> &'static str {
    "docs"
}

#[get("/help")]
pub fn help() -> &'static str {
    "help"
}

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}
