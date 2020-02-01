use crate::Db;

#[get("/events")]
pub fn list(_conn: Db) -> &'static str {
    "all events"
}
