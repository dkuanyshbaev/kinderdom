// use super::schema::profiles;
// use diesel::sql_types::Timestamp;
// use diesel::{self, prelude::*};

// pub mod schema;

mod schema {
    table! {
        profiles (id) {
            id -> Int4,
            name -> Varchar,
            photo -> Varchar,
            video -> Varchar,
            description -> Text,
            published -> Bool,
            // created_at -> Timestamp,
        }
    }
}

use self::schema::profiles;

#[table_name = "profiles"]
#[derive(Insertable)]
pub struct NewProfile<'a> {
    pub name: &'a str,
    pub photo: &'a str,
    pub video: &'a str,
    pub description: &'a str,
    pub published: bool,
}

// why &'a str ^????

#[derive(Queryable, Debug)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub photo: String,
    pub video: String,
    pub description: String,
    pub published: bool,
    // pub created_at: Timestamp,
}

// use super::Config;
// use diesel::pg::PgConnection;
use diesel::pg::PgConnection;
use diesel::prelude::*;

// pub mod models;
// pub mod schema;

// pub fn establish_connection(config: Config) -> PgConnection {
//     PgConnection::establish(&config.db).expect(&format!("Error connecting to {}", config.db))
// }

// pub fn create_profile<'a>(connection: &PgConnection, name: &'a str) {
//     // let task = models::NewTask { title };
//     //
//     // diesel::insert_into(schema::task::table)
//     //     .values(&task)
//     //     .execute(connection)
//     //     .expect("Error inserting new task");
//     println!("create profile");
// }

pub fn query_profile(connection: &PgConnection) -> Vec<Profile> {
    schema::profiles::table
        .load::<Profile>(connection)
        .expect("Error loading profiles")
}
