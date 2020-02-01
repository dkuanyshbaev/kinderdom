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
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::{env, process};
use views::{admin, pages};

pub mod models;
pub mod views;

#[database("kinderdom")]
pub struct Db(PgConnection);

pub struct Config {
    pub secret: String,
}

impl Config {
    pub fn new() -> Result<Config, env::VarError> {
        let secret = env::var("SECRET")?;
        Ok(Config { secret })
    }
}

fn rocket(_config: Config) -> Rocket {
    rocket::ignite()
        .attach(Db::fairing())
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![
                pages::index,
                pages::profiles,
                pages::projects,
                pages::events,
                pages::about,
            ],
        )
        .mount(
            "/admin",
            routes![
                // profiles
                admin::profiles::list,
                admin::profiles::show,
                admin::profiles::create,
                admin::profiles::update,
                admin::profiles::delete,
                // projects
                admin::projects::list,
                admin::projects::show,
                admin::projects::create,
                admin::projects::update,
                admin::projects::delete,
                // articles
                admin::articles::list,
                admin::articles::show,
                admin::articles::create,
                admin::articles::update,
                admin::articles::delete,
                // events
                admin::events::list,
                admin::events::show,
                admin::events::create,
                admin::events::update,
                admin::events::delete,
            ],
        )
        .register(catchers![pages::not_found])
}

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing config: {}", err);
        process::exit(1);
    });

    let error = rocket(config).launch();
    println!("Launch failed: {}", error);
}
