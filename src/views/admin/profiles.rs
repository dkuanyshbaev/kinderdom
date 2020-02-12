use crate::errors::Error as ApiError;
use crate::models::profile::{Profile, Profiles};
use crate::Db;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
// use rocket::response::status::{Created, NoContent};

#[get("/profiles")]
pub fn list(connection: Db) -> Template {
    // let profiles = Profile::all(&connection);
    // let context = Profiles { profiles: profiles };

    // let context = Profiles {
    //     profiles: Profile::all(&connection),
    // };
    // Template::render("admin/profiles/list", &context)

    Template::render(
        "admin/profiles/list",
        Profiles {
            profiles: Profile::all(&connection),
        },
    )
}

//----------------------------------------------------------------------------

// use rocket::response::status::NotFound;
//
// #[get("/notes/<id>", format = "application/json")]
// fn note_get(db: DB, id: i32) -> Result<JSON<Note>, ApiError> {
//     let note = get_note(db.conn(), id)?;
//         Ok(JSON(note))
//         }
// }

// #[get("/notes/<id>", format = "application/json")]
// fn note_get(db: DB, id: i32) -> Result<JSON<Note>, diesel::result::Error> {
//     let note = get_note(db.conn(), id);
//     match note {
//         Ok(note) => Ok(JSON(note)),
//         Err(err) => Err(err),
//     }
// }

// pub fn show(connection: Db, id: i32) -> Result<Template, diesel::result::Error> {

#[get("/profiles/<id>")]
pub fn show(connection: Db, id: i32) -> Result<Template, ApiError> {
    let profile = Profile::get(&connection, id)?;

    Ok(Template::render("admin/profiles/show", profile))
}

//----------------------------------------------------------------------------

#[get("/profiles/add")]
pub fn add(connection: Db) -> Template {
    let new_profile = Profile::new(&connection);

    Template::render("admin/profiles/form", new_profile)
}

#[post("/profiles")]
pub fn create(_conn: Db) -> &'static str {
    "post create profile"
    // Ok(Created(url, Some(JSON(note))))
}

#[put("/profiles/<id>")]
pub fn update(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "update profile"
}

// te_delete(db: DB, id: i32) -> Result<NoContent, ApiError> {}
#[delete("/profiles/<id>")]
pub fn delete(connection: Db, id: i32) -> Redirect {
    let _ok = Profile::delete(&connection, id);

    // Redirect::to(uri!(get: name = "Unknown"))
    Redirect::to("/admin/profiles")
    // Ok(NoContent)
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
