// use diesel::sql_types::Timestamp;
// use self::schema::profiles;
use crate::models::schema::profiles;
use diesel::prelude::*;

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

// pub fn create_profile<'a>(connection: &PgConnection, name: &'a str) {
//     // let task = models::NewTask { title };
//     //
//     // diesel::insert_into(schema::task::table)
//     //     .values(&task)
//     //     .execute(connection)
//     //     .expect("Error inserting new task");
//     println!("create profile");
// }

pub fn all_profiles(connection: &PgConnection) -> Vec<Profile> {
    crate::models::schema::profiles::table
        .load::<Profile>(connection)
        .expect("Error loading profiles")
}
