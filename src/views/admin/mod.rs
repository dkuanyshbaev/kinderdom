pub mod articles;
pub mod events;
pub mod profiles;
pub mod projects;

use crate::auth::LoginForm;
use crate::Db;
use crate::KinderResult;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

#[get("/")]
pub fn main(_conn: Db) -> Template {
    let context = TemplateContext {
        name: "main admin page".to_string(),
        items: vec!["one", "two"],
    };
    Template::render("admin/main", &context)
}

//--------------------------------------------------------------------------

#[get("/login")]
pub fn login_page(_conn: Db) -> Template {
    let context = TemplateContext {
        name: "login page".to_string(),
        items: vec!["one", "two"],
    };
    Template::render("admin/login", &context)
}

#[post("/login", data = "<login_form>")]
// fn login(mut cookies: Cookies, login_form: Form<LoginForm>) -> Result<Redirect, Flash<Redirect>> {
pub fn login(mut cookies: Cookies, login_form: Form<LoginForm>) -> KinderResult<Redirect> {
    // if login.username == "Sergio" && login.password == "password" {
    if login_form.password == "assword" {
        cookies.add_private(Cookie::new("admin_id", 1.to_string()));
        // Ok(Redirect::to(uri!(index)))
        Ok(Redirect::to("/admin"))
    } else {
        // Redirect::to(uri!(login_page))
        Ok(Redirect::to("/admin/login"))
        // Err(Flash::error(
        //     Redirect::to(uri!(login_page)),
        //     "Invalid username/password.",
        // ))
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("admin_id"));

    Redirect::to("/admin/login")
}
