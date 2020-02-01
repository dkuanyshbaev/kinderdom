use crate::Db;

#[get("/projects")]
pub fn list(_conn: Db) -> &'static str {
    "all projects"
}

#[get("/projects/<id>")]
pub fn show(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "project details"
}

#[post("/projects")]
pub fn create(_conn: Db) -> &'static str {
    "create project"
}

#[put("/projects/<id>")]
pub fn update(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "update project"
}

#[delete("/projects/<id>")]
pub fn delete(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "delete project"
}
