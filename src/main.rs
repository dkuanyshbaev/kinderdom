#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

use diesel::PgConnection;
use rocket::Rocket;
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::{env, process};
use views::{admin, pages};

pub mod auth;
pub mod errors;
pub mod models;
pub mod views;

type KinderResult<T> = Result<T, errors::KinderError>;

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

fn rocket(config: Config) -> Rocket {
    rocket::ignite()
        .attach(Db::fairing())
        .attach(Template::fairing())
        .manage(config)
        .mount("/static", StaticFiles::from("static/"))
        .mount("/admin/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![
                pages::index,
                pages::articles,
                pages::profiles,
                pages::projects,
                pages::events,
                pages::about,
                pages::docs,
                pages::help,
            ],
        )
        .mount(
            "/admin",
            routes![
                // common
                admin::main,
                admin::login_page,
                admin::login,
                admin::logout,
                // articles
                admin::articles::list,
                // admin::articles::show,
                // admin::articles::create,
                // admin::articles::update,
                // admin::articles::delete,
                // profiles
                admin::profiles::list,
                admin::profiles::add,
                admin::profiles::create,
                admin::profiles::edit,
                admin::profiles::update,
                admin::profiles::delete,
                // projects
                admin::projects::list,
                admin::projects::show,
                admin::projects::create,
                admin::projects::update,
                admin::projects::delete,
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

#[cfg(test)]
mod test {
    use super::{rocket, Config};
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn test_main() {
        let test_config = Config {
            secret: "thesecret".to_string(),
        };
        let client = Client::new(rocket(test_config)).unwrap();

        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let body = response.body_string().unwrap();
        assert!(body.contains("kinderdom@mail.ru"));
    }
}
