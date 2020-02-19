pub mod articles;
pub mod events;
pub mod profiles;
pub mod projects;

use crate::auth::LoginForm;
use crate::{views::NoContext, Config, KinderResult};
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn main() -> Redirect {
    Redirect::to("/admin/articles")
}

#[get("/login")]
pub fn login_page() -> Template {
    Template::render("admin/login", NoContext {})
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
        Ok(Redirect::to("/admin/login"))
    }
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("admin"));

    Redirect::to("/admin/login")
}
