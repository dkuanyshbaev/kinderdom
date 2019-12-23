#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use diesel::pg::PgConnection;
use diesel::prelude::*;
// use rocket_contrib::databases::diesel;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::env;

pub mod db;
pub mod handlers;

struct Db(diesel::PgConnection);

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

pub fn run(config: Config) -> Result<(), String> {
    println!("{}", config.secret);
    // let connection = PgConnection::establish(&config.db).expect(&format!("Error connecting to db"));

    let error = rocket::ignite()
        // .manage(config)
        .attach(Db::fairing())
        .mount("/", routes![handlers::index, handlers::about])
        .mount("/admin", routes![handlers::profiles, handlers::news])
        .mount("/static", StaticFiles::from("static/"))
        .attach(Template::fairing())
        .register(catchers![handlers::not_found])
        .launch();
    Err(error.to_string())
}
