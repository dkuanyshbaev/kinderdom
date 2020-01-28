#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use rocket_contrib::databases::diesel;
use diesel::PgConnection;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::env;

pub mod db;
pub mod handlers;

#[database("kinderdom")]
struct Db(PgConnection);

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
        .mount("/", routes![index, handlers::about])
        .mount("/admin", routes![handlers::profiles, handlers::news])
        .mount("/static", StaticFiles::from("static/"))
        .attach(Template::fairing())
        .register(catchers![handlers::not_found])
        .launch();
    Err(error.to_string())
}

#[get("/")]
// pub fn index(config: State<Config>) -> Template {
fn index(conn: Db) -> Template {
    // let secret = &config.secret;

    // println!("------->>>>>> {}", secret);

    // -----------------------------------------------------------
    // let results = posts
    //     .filter(published.eq(true))
    //     .limit(5)
    //     .load::<Post>(&connection)
    //     .expect("Error loading posts");
    //
    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.title);
    //     println!("----------\n");
    //     println!("{}", post.body);
    // }
    let ps = db::query_profile(&conn);
    for p in ps {
        println!("profile: {:?}", p);
    }
    // -----------------------------------------------------------

    let name = "Denis".to_string();
    let context = handlers::TemplateContext {
        name,
        items: vec!["One", "Two", "Three"],
    };
    Template::render("index", &context)
}
