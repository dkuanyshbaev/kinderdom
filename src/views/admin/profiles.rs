use crate::models::profile::{NewProfile, Profile, Profiles};
use crate::{Db, KinderResult};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

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

#[get("/profiles/add")]
pub fn add() -> Template {
    // TODO: real context
    let name = "Анкеты".to_string();

    Template::render("admin/profiles/add", TemplateContext { name })
}

#[post("/profiles", data = "<new_profile>")]
pub fn create(connection: Db, new_profile: Form<NewProfile>) -> KinderResult<Redirect> {
    let profile = Profile::insert(&connection, new_profile.into_inner())?;

    Ok(Redirect::to(format!("/admin/profiles/{}", profile.id)))
}

#[get("/profiles/<id>")]
pub fn edit(connection: Db, id: i32) -> KinderResult<Template> {
    let profile = Profile::get(&connection, id)?;

    Ok(Template::render("admin/profiles/edit", profile))
}

#[put("/profiles/<id>", data = "<new_profile>")]
pub fn update(connection: Db, new_profile: Form<NewProfile>, id: i32) -> KinderResult<Redirect> {
    let profile = Profile::update(&connection, new_profile.into_inner(), id)?;

    Ok(Redirect::to(format!("/admin/profiles/{}", profile.id)))
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
