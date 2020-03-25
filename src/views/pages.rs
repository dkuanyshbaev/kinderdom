use crate::auth::{Admin, LoginForm};
use crate::models::cause::Cause;
use crate::models::event::Event;
use crate::models::report::Report;
use crate::views::{ListContext, NoContext};
use crate::{Config, Db, KinderResult};
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
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

#[get("/admin")]
pub fn admin(_admin: Admin) -> Redirect {
    Redirect::to("/admin/events")
}

#[get("/login")]
pub fn login_page() -> Template {
    Template::render("pages/login", NoContext {})
}

#[post("/login", data = "<login_form>")]
pub fn login(
    mut cookies: Cookies,
    config: State<Config>,
    login_form: Form<LoginForm>,
) -> KinderResult<Redirect> {
    if login_form.password == config.secret {
        cookies.add_private(Cookie::new("admin", 1.to_string()));

        Ok(Redirect::to("/admin"))
    } else {
        Ok(Redirect::to("/login"))
    }
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("admin"));

    Redirect::to("/login")
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("pages/404", NoContext {})
}

#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to("/login")
}
