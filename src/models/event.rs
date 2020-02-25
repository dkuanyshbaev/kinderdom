use super::schema::events;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "events"]
pub struct NewEvent {
    pub published: bool,
}

#[derive(Serialize, Queryable, Debug)]
pub struct Event {
    pub id: i32,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

impl Event {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Event>> {
        events::table.order(events::id.desc()).load(connection)
    }

    pub fn published(connection: &PgConnection) -> QueryResult<Vec<Event>> {
        events::table
            .filter(events::published.eq(true))
            .limit(4)
            .order(events::id.desc())
            .load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Event> {
        events::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, event: NewEvent) -> QueryResult<Event> {
        diesel::insert_into(events::table)
            .values(event)
            .get_result(connection)
    }

    pub fn update(connection: &PgConnection, event: NewEvent, id: i32) -> QueryResult<Event> {
        diesel::update(events::table.find(id))
            .set(event)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Event> {
        diesel::delete(events::table.find(id)).get_result(connection)
    }
}
