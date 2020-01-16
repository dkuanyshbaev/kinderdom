use super::schema::profiles;
use diesel::sql_types::Timestamp;

#[derive(Insertable)]
#[table_name = "profiles"]
pub struct NewProfile<'a> {
    pub name: &'a str,
    pub photo: &'a str,
    pub video: &'a str,
    pub description: &'a str,
    pub published: bool,
}

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
