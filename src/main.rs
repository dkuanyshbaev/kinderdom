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
extern crate rocket_multipart_form_data;

use config::Config;
use diesel::PgConnection;
use rocket::Rocket;
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::process;
use views::{admin, common, pages};

mod auth;
mod config;
mod errors;
mod models;
mod views;

type KinderResult<T> = Result<T, errors::KinderError>;

#[database("kinderdom")]
pub struct Db(PgConnection);

fn rocket(config: Config) -> Rocket {
    rocket::ignite()
        .attach(Db::fairing())
        .attach(Template::fairing())
        .manage(config)
        .mount("/static", StaticFiles::from("static/"))
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
                common::main,
                common::login_page,
                common::login,
                common::logout,
            ],
        )
        .mount(
            "/admin/articles",
            routes![
                admin::articles::list,
                admin::articles::add,
                admin::articles::create,
                admin::articles::edit,
                admin::articles::update,
                admin::articles::delete,
            ],
        )
        .mount(
            "/admin/profiles",
            routes![
                admin::profiles::list,
                admin::profiles::add,
                admin::profiles::create,
                admin::profiles::edit,
                admin::profiles::update,
                admin::profiles::delete,
            ],
        )
        .mount(
            "/admin/projects",
            routes![
                admin::projects::list,
                admin::projects::add,
                admin::projects::create,
                admin::projects::edit,
                admin::projects::update,
                admin::projects::delete,
            ],
        )
        .mount(
            "/admin/events",
            routes![
                admin::events::list,
                admin::events::add,
                admin::events::create,
                admin::events::edit,
                admin::events::update,
                admin::events::delete,
            ],
        )
        .mount(
            "/admin/docs",
            routes![
                admin::docs::list,
                admin::docs::add,
                admin::docs::create,
                admin::docs::edit,
                admin::docs::update,
                admin::docs::delete,
            ],
        )
        .mount(
            "/admin/donors",
            routes![
                admin::donors::list,
                admin::donors::add,
                admin::donors::create,
                admin::donors::edit,
                admin::donors::update,
                admin::donors::delete,
            ],
        )
        .mount(
            "/admin/orgs",
            routes![
                admin::orgs::list,
                admin::orgs::add,
                admin::orgs::create,
                admin::orgs::edit,
                admin::orgs::update,
                admin::orgs::delete,
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
