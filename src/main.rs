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
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use views::{admin, pages};

mod auth;
mod config;
mod errors;
mod models;
mod views;

type KinderResult<T> = Result<T, errors::KinderError>;

#[database("kinderdom")]
pub struct Db(diesel::PgConnection);

fn rocket(config: Config) -> rocket::Rocket {
    rocket::ignite()
        .attach(Db::fairing())
        .attach(Template::fairing())
        .manage(config)
        .mount("/static", StaticFiles::from("static/"))
        .mount(
            "/",
            routes![
                pages::index,
                pages::events,
                pages::event_detail,
                pages::causes,
                pages::cause_detail,
                pages::reports,
                pages::about,
                pages::help,
                pages::admin,
                pages::login_page,
                pages::login,
                pages::logout,
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
            "/admin/cats",
            routes![
                admin::cats::list,
                admin::cats::add,
                admin::cats::create,
                admin::cats::edit,
                admin::cats::update,
                admin::cats::delete,
            ],
        )
        .mount(
            "/admin/causes",
            routes![
                admin::causes::list,
                admin::causes::add,
                admin::causes::create,
                admin::causes::edit,
                admin::causes::update,
                admin::causes::delete,
            ],
        )
        .mount(
            "/admin/reports",
            routes![
                admin::reports::list,
                admin::reports::add,
                admin::reports::create,
                admin::reports::edit,
                admin::reports::update,
                admin::reports::delete,
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
        .register(catchers![pages::not_found, pages::unauthorized])
}

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing config: {}", err);
        std::process::exit(1);
    });

    let error = rocket(config).launch();
    println!("Launch failed: {}", error);
}

// TODO: write the actual tests
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
