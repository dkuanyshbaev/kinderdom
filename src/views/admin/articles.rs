use crate::Db;

#[get("/articles")]
pub fn list(_conn: Db) -> &'static str {
    "all articles"
}

#[get("/articles/<id>")]
pub fn show(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "article details"
}

#[post("/articles")]
pub fn create(_conn: Db) -> &'static str {
    "create article"
}

#[put("/articles/<id>")]
pub fn update(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "update article"
}

#[delete("/articles/<id>")]
pub fn delete(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "delete article"
}
