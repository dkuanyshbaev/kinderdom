// use diesel::sql_types::Timestamp;
use super::schema::profiles;
use diesel::prelude::*;

#[table_name = "profiles"]
#[derive(Serialize, Queryable, Insertable, FromForm, AsChangeset, Debug, Clone)]
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
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Profile>> {
        profiles::table
            .order(profiles::id.desc())
            .load::<Profile>(connection)
    }

    pub fn published(connection: &PgConnection) -> QueryResult<Vec<Profile>> {
        profiles::table
            .filter(profiles::published.eq(true))
            .limit(4)
            .order(profiles::id.desc())
            .load::<Profile>(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Profile> {
        profiles::table.find(id).get_result::<Profile>(connection)
    }

    pub fn insert(connection: &PgConnection, profile: Profile) -> QueryResult<Profile> {
        diesel::insert_into(profiles::table)
            .values(profile)
            .get_result::<Profile>(connection)
    }

    pub fn update(connection: &PgConnection, profile: Profile, id: i32) -> QueryResult<Profile> {
        // let old_profile = profiles::table.find(id).get_result::<Profile>(connection);
        //
        // if old_profile.is_err() {
        //     // return
        // }

        diesel::update(profiles::table.find(id))
            .set(profile)
            .get_result::<Profile>(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Profile> {
        diesel::delete(profiles::table.find(id)).get_result::<Profile>(connection)
    }
}
