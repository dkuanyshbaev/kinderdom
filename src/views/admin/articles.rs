use crate::Db;

#[get("/articles")]
pub fn list(_conn: Db) -> &'static str {
    "all articles"
}
