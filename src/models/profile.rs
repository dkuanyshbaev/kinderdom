use super::schema::profiles;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "profiles"]
pub struct NewProfile {
    pub name: String,
    pub photo: String,
    pub video: String,
    pub description: String,
    pub published: bool,
}

#[derive(Serialize, Queryable, Debug)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub photo: String,
    pub video: String,
    pub description: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

impl Profile {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Profile>> {
        profiles::table.order(profiles::id.desc()).load(connection)
    }

    pub fn published(connection: &PgConnection) -> QueryResult<Vec<Profile>> {
        profiles::table
            .filter(profiles::published.eq(true))
            .limit(4)
            .order(profiles::id.desc())
            .load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Profile> {
        profiles::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, profile: NewProfile) -> QueryResult<Profile> {
        diesel::insert_into(profiles::table)
            .values(profile)
            .get_result(connection)
    }

    pub fn update(connection: &PgConnection, profile: NewProfile, id: i32) -> QueryResult<Profile> {
        diesel::update(profiles::table.find(id))
            .set(profile)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Profile> {
        diesel::delete(profiles::table.find(id)).get_result(connection)
    }
}
