use crate::Db;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

#[get("/projects")]
pub fn list(_conn: Db) -> Template {
    let context = TemplateContext {
        name: "projects list".to_string(),
        items: vec!["one", "two"],
    };
    Template::render("admin/projects/list", &context)
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
