use crate::Db;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

#[get("/articles")]
pub fn list(_conn: Db) -> Template {
    let context = TemplateContext {
        name: "articles list".to_string(),
        items: vec!["one", "two"],
    };
    Template::render("admin/articles/list", &context)
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
