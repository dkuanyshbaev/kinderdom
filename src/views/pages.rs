use crate::models::{article::Article, event::Event, profile::Profile, project::Project};
use crate::Db;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

#[get("/")]
pub fn index(connection: Db) -> Template {
    let _ = Article::published(&connection);
    let _ = Profile::published(&connection);
    let _ = Project::published(&connection);
    let _ = Event::published(&connection);

    let name = "Denis".to_string();
    let context = TemplateContext {
        name,
        items: vec!["One", "Two", "Three"],
    };
    Template::render("pages/index", &context)
}

#[get("/events")]
pub fn events() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/events", &context)
}

#[get("/events/<id>")]
pub fn event_detail(id: i32) -> Template {
    println!("id: {}", id);
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/event_detail", &context)
}

#[get("/causes")]
pub fn causes() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/causes", &context)
}

#[get("/causes/<id>")]
pub fn cause_detail(id: i32) -> Template {
    println!("id: {}", id);
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/cause_detail", &context)
}

#[get("/reports")]
pub fn reports() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/reports", &context)
}

#[get("/about")]
pub fn about() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/about", &context)
}

#[get("/help")]
pub fn help() -> Template {
    let context = TemplateContext {
        name: "".to_string(),
        items: vec![],
    };
    Template::render("pages/help", &context)
}
