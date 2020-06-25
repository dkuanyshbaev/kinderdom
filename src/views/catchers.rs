use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
pub struct NoContext {}

#[catch(500)]
pub fn internal_error() -> Template {
    Template::render("pages/500", NoContext {})
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("pages/404", NoContext {})
}

#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to("/login")
}

#[catch(422)]
pub fn unprocessable() -> Redirect {
    Redirect::to("/")
}
