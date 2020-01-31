#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use diesel::PgConnection;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::env;
use std::process;

pub mod models;
pub mod routes;

#[database("kinderdom")]
pub struct Db(PgConnection);

pub struct Config {
    pub db: String,
    pub secret: String,
}

impl Config {
    pub fn new() -> Result<Config, env::VarError> {
        let db = env::var("DATABASE_URL")?;
        let secret = env::var("SECRET")?;
        Ok(Config { db, secret })
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(Db::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount("/", routes![routes::index])
        // .mount("/todo", routes![new, toggle, delete])
        .attach(Template::fairing())
        .register(catchers![routes::not_found])
}

fn main() {
    let _config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing config: {}", err);
        process::exit(1);
    });

    // if let Err(e) = kinderdom::run(config) {
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // }

    rocket().launch();
}
