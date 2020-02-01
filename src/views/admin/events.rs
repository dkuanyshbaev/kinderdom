use crate::Db;

#[get("/events")]
pub fn list(_conn: Db) -> &'static str {
    "all events"
}

#[get("/events/<id>")]
pub fn show(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "event details"
}

#[post("/events")]
pub fn create(_conn: Db) -> &'static str {
    "create event"
}

#[put("/events/<id>")]
pub fn update(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "update event"
}

#[delete("/events/<id>")]
pub fn delete(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "delete event"
}
