// extern crate rocket_contrib;
// extern crate serde_json;

// use super::Config;
use crate::Db;
use rocket::Request;
// use rocket::State;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

// pub mod models;
// use models::Profile;
// use crate::models::profile::all_profiles;
use crate::models::profile::Profile;

#[derive(Serialize)]
pub struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

#[get("/")]
// pub fn index(config: State<Config>) -> Template {
pub fn index(conn: Db) -> Template {
    // let secret = &config.secret;
    // println!("------->>>>>> {}", secret);
    // -----------------------------------------------------------

    // let ps = all_profiles(&conn);
    let ps = Profile::all(&conn);
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

// ----------------------------------------------------
// #[database("my_db")]
// struct MyDatabase(diesel::SqliteConnection);
//
// fn load_from_db(conn: &diesel::SqliteConnection) -> Data {
//     // Do something with connection, return some data.
// }
//
// #[get("/")]
// fn my_handler(conn: MyDatabase) -> Data {
//     load_from_db(&*conn)
// }
// ----------------------------------------------------

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

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}
