use crate::auth::{Admin, LoginForm};
use crate::{Config, KinderResult};
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
pub struct NoContext {}

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

#[get("/admin")]
pub fn admin(_admin: Admin) -> Redirect {
    Redirect::to("/admin/events")
}
