use super::Config;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod models;
pub mod schema;

pub fn establish_connection(config: Config) -> PgConnection {
    PgConnection::establish(&config.db).expect(&format!("Error connecting to {}", config.db))
}

// pub fn create_profile<'a>(connection: &PgConnection, name: &'a str) {
//     // let task = models::NewTask { title };
//     //
//     // diesel::insert_into(schema::task::table)
//     //     .values(&task)
//     //     .execute(connection)
//     //     .expect("Error inserting new task");
//     println!("create profile");
// }

pub fn query_profile(connection: &PgConnection) -> Vec<models::Profile> {
    schema::profiles::table
        .load::<models::Profile>(connection)
        .expect("Error loading profiles")
}
