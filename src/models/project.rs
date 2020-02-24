use super::schema::projects;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "projects"]
pub struct NewProject {
    pub published: bool,
}

#[derive(Serialize, Queryable, Debug)]
pub struct Project {
    pub id: i32,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

impl Project {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Project>> {
        projects::table.order(projects::id.desc()).load(connection)
    }

    // pub fn published(connection: &PgConnection) -> QueryResult<Vec<Project>> {
    //     projects::table
    //         .filter(projects::published.eq(true))
    //         .limit(4)
    //         .order(projects::id.desc())
    //         .load(connection)
    // }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Project> {
        projects::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, project: NewProject) -> QueryResult<Project> {
        diesel::insert_into(projects::table)
            .values(project)
            .get_result(connection)
    }

    pub fn update(connection: &PgConnection, project: NewProject, id: i32) -> QueryResult<Project> {
        diesel::update(projects::table.find(id))
            .set(project)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Project> {
        diesel::delete(projects::table.find(id)).get_result(connection)
    }
}
