use crate::Db;

#[get("/profiles")]
pub fn profiles(_conn: Db) -> &'static str {
    "profiles"
}
