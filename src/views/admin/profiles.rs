use crate::errors::KinderError;
use crate::models::profile::{Profile, Profiles};
use crate::Db;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

type KinderResult<T> = Result<T, KinderError>;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/profiles")]
pub fn list(connection: Db) -> KinderResult<Template> {
    let profiles = Profile::all(&connection)?;

    Ok(Template::render(
        "admin/profiles/list",
        Profiles { profiles },
    ))
}

#[get("/profiles/<id>")]
pub fn show(connection: Db, id: i32) -> KinderResult<Template> {
    let profile = Profile::get(&connection, id)?;

    Ok(Template::render("admin/profiles/show", profile))
}

#[get("/profiles/add")]
pub fn add() -> Template {
    // TODO: real context
    let name = "Анкеты".to_string();

    Template::render("admin/profiles/form", TemplateContext { name })
}

#[post("/profiles", data = "<profile>")]
pub fn create(connection: Db, profile: Form<Profile>) -> KinderResult<Redirect> {
    let profile = Profile::insert(&connection, profile.into_inner())?;

    if let Some(id) = profile.id {
        Ok(Redirect::to(format!("/admin/profiles/{}", id)))
    } else {
        Err(KinderError::InternalServerError)
    }
}

#[put("/profiles/<id>")]
pub fn update(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "update profile"
}

#[delete("/profiles/<id>")]
pub fn delete(connection: Db, id: i32) -> KinderResult<Redirect> {
    let _profile = Profile::delete(&connection, id)?;

    Ok(Redirect::to("/admin/profiles"))
}

//--------------------------------------------------------------------------
//
// #[get("/admin")]
// fn admin_panel(admin: AdminUser) -> &'static str {
//     "Hello, administrator. This is the admin panel!"
// }
//
// #[get("/admin", rank = 2)]
// fn admin_panel_user(user: User) -> &'static str {
//     "Sorry, you must be an administrator to access this page."
// }
//
// #[get("/admin", rank = 3)]
// fn admin_panel_redirect() -> Redirect {
//     Redirect::to("/login")
// }
//
//--------------------------------------------------------------------------
//
// /// Retrieve the user's ID, if any.
// #[get("/user_id")]
// fn user_id(cookies: Cookies) -> Option<String> {
//     cookies.get_private("user_id")
//         .map(|cookie| format!("User ID: {}", cookie.value()))
// }
//
// /// Remove the `user_id` cookie.
// #[post("/logout")]
// fn logout(mut cookies: Cookies) -> Flash<Redirect> {
//     cookies.remove_private(Cookie::named("user_id"));
//     Flash::success(Redirect::to("/"), "Successfully logged out.")
// }
