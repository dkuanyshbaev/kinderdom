// use diesel::sql_types::Timestamp;
use crate::models::schema::profiles;
use crate::models::schema::profiles::dsl::profiles as all_profiles;
use diesel::prelude::*;

#[table_name = "profiles"]
#[derive(Serialize, Queryable, Insertable, FromForm, Debug, Clone)]
pub struct Profile {
    pub id: Option<i32>,
    pub name: String,
    pub photo: String,
    pub video: String,
    pub description: String,
    pub published: bool,
    // pub created_at: Timestamp,
}

#[derive(Serialize)]
pub struct Profiles {
    pub profiles: Vec<Profile>,
}

impl Profile {
    pub fn all(connection: &PgConnection) -> Vec<Profile> {
        all_profiles
            .filter(profiles::published.eq(false))
            .limit(8)
            .order(profiles::id.desc())
            .load::<Profile>(connection)
            .unwrap()
    }

    pub fn get(connection: &PgConnection, id: i32) -> Profile {
        all_profiles
            .find(id)
            .get_result::<Profile>(connection)
            .unwrap()
    }

    pub fn new(connection: &PgConnection) -> Profile {
        let empty_profile = Profile {
            id: None,
            name: "".to_string(),
            photo: "".to_string(),
            video: "".to_string(),
            description: "".to_string(),
            published: false,
        };

        diesel::insert_into(profiles::table)
            .values(empty_profile)
            .get_result(connection)
            .expect("Error saving new profile")
        // .execute(connection)
        // .is_ok()
    }

    pub fn delete(connection: &PgConnection, id: i32) -> bool {
        diesel::delete(all_profiles.find(id))
            .execute(connection)
            .is_ok()
    }

    // pub fn toggle_with_id(id: i32, conn: &SqliteConnection) -> bool {
    //     let task = all_tasks.find(id).get_result::<Task>(conn);
    //     if task.is_err() {
    //         return false;
    //     }
    //
    //     let new_status = !task.unwrap().completed;
    //     let updated_task = diesel::update(all_tasks.find(id));
    //     updated_task
    //         .set(task_completed.eq(new_status))
    //         .execute(conn)
    //         .is_ok()
    // }
}

// #[table_name = "profiles"]
// #[derive(Insertable)]
// pub struct NewProfile<'a> {
//     pub name: &'a str,
//     pub photo: &'a str,
//     pub video: &'a str,
//     pub description: &'a str,
//     pub published: bool,
// }

// why &'a str ^????

// #[derive(Queryable, Debug)]
// pub struct Profile {
//     pub id: i32,
//     pub name: String,
//     pub photo: String,
//     pub video: String,
//     pub description: String,
//     pub published: bool,
//     // pub created_at: Timestamp,
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

// pub fn all_profiles(connection: &PgConnection) -> Vec<Profile> {
//     crate::models::schema::profiles::table
//         .load::<Profile>(connection)
//         .expect("Error loading profiles")
// }
