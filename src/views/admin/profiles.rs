use crate::models::profile::{NewProfile, Profile, Profiles};
use crate::{auth::Admin, Db, KinderResult};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/profiles")]
// pub fn list(_admin: Admin, connection: Db) -> KinderResult<Template> {
pub fn list(connection: Db) -> KinderResult<Template> {
    let profiles = Profile::all(&connection)?;

    Ok(Template::render(
        "admin/profiles/list",
        Profiles { profiles },
    ))
}

#[get("/profiles/add")]
pub fn add() -> Template {
    let name = "Анкеты".to_string();

    Template::render("admin/profiles/add", TemplateContext { name })
}

#[post("/profiles", data = "<new_profile>")]
pub fn create(connection: Db, new_profile: Form<NewProfile>) -> KinderResult<Redirect> {
    let _profile = Profile::insert(&connection, new_profile.into_inner())?;

    // Ok(Redirect::to(format!("/admin/profiles/{}", profile.id)))
    Ok(Redirect::to("/admin/profiles"))
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
