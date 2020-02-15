pub mod articles;
pub mod events;
pub mod profiles;
pub mod projects;

use crate::auth::LoginForm;
use crate::{Config, KinderResult};
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/")]
pub fn main() -> Template {
    let name = "Dashboard".to_string();

    Template::render("admin/main", TemplateContext { name })
}

#[get("/login")]
pub fn login_page() -> Template {
    let name = "Login".to_string();

    Template::render("admin/login", TemplateContext { name })
}

#[post("/login", data = "<login_form>")]
pub fn login(
    mut cookies: Cookies,
    config: State<Config>,
    login_form: Form<LoginForm>,
) -> KinderResult<Redirect> {
    if login_form.password == config.secret {
        cookies.add_private(Cookie::new("admin_id", 1.to_string()));
        Ok(Redirect::to("/admin"))
    } else {
        Ok(Redirect::to("/admin/login"))
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("admin_id"));

    Redirect::to("/admin/login")
}
