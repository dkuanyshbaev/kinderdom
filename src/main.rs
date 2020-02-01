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
                views::site::index,
                views::site::profiles,
                views::site::projects,
                views::site::events,
                views::site::about,
            ],
        )
        .mount(
            "/admin",
            routes![
                // profiles
                views::admin::profiles::list,
                views::admin::profiles::show,
                views::admin::profiles::create,
                views::admin::profiles::update,
                views::admin::profiles::delete,
                // projects
                views::admin::projects::list,
                views::admin::projects::show,
                views::admin::projects::create,
                views::admin::projects::update,
                views::admin::projects::delete,
                // articles
                views::admin::articles::list,
                views::admin::articles::show,
                views::admin::articles::create,
                views::admin::articles::update,
                views::admin::articles::delete,
                // events
                views::admin::events::list,
                views::admin::events::show,
                views::admin::events::create,
                views::admin::events::update,
                views::admin::events::delete,
            ],
        )
        .register(catchers![views::site::not_found])
}

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing config: {}", err);
        process::exit(1);
    });

    let error = rocket(config).launch();
    println!("Launch failed: {}", error);
}
