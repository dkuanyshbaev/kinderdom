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
use views::{admin, catchers, en, login, pages};

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
                pages::event_details,
                pages::causes,
                pages::cause_details,
                pages::profiles,
                pages::profile_details,
                pages::reports,
                pages::about,
                pages::search,
                pages::payment,
                pages::thankyou,
            ],
        )
        .mount(
            "/en",
            routes![
                en::index_en,
                // pages::events,
                // pages::event_details,
                // pages::causes,
                // pages::cause_details,
                // pages::profiles,
                // pages::profile_details,
                // pages::reports,
                // pages::about,
                // pages::search,
                // pages::payment,
                // pages::thankyou,
            ],
        )
        .mount(
            "/",
            routes![login::login_page, login::login, login::logout, login::admin],
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
        .register(catchers![
            catchers::not_found,
            catchers::unauthorized,
            catchers::unprocessable,
            catchers::internal_error
        ])
}

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing config: {}", err);
        std::process::exit(1);
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
