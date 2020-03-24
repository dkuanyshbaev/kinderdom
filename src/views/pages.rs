use crate::models::cause::Cause;
use crate::models::event::Event;
use crate::models::report::Report;
use crate::views::{ListContext, NoContext};
use crate::{Db, KinderResult};
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(_connection: Db) -> KinderResult<Template> {
    // TODO: get all
    Ok(Template::render("pages/index", NoContext {}))
}

#[get("/events")]
pub fn events(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/events",
        ListContext {
            items: Event::published(&connection)?,
        },
    ))
}

#[get("/events/<id>")]
pub fn event_detail(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/event_detail",
        Event::get(&connection, id)?,
    ))
}

#[get("/causes")]
pub fn causes(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/causes",
        ListContext {
            items: Cause::published(&connection)?,
        },
    ))
}

#[get("/causes/<id>")]
pub fn cause_detail(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/cause_detail",
        Cause::get(&connection, id)?,
    ))
}

#[get("/reports")]
pub fn reports(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/reports",
        ListContext {
            items: Report::all(&connection)?,
        },
    ))
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("pages/about", NoContext {})
}

#[get("/help")]
pub fn help() -> Template {
    Template::render("pages/help", NoContext {})
}
