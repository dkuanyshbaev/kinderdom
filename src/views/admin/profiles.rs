use crate::Db;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

#[get("/profiles")]
pub fn list(_conn: Db) -> Template {
    let context = TemplateContext {
        name: "profiles list".to_string(),
        items: vec!["one", "two"],
    };
    Template::render("admin/profiles/list", &context)
}

#[get("/profiles/<id>")]
pub fn show(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "profile details"
}

#[post("/profiles")]
pub fn create(_conn: Db) -> &'static str {
    "create profile"
}

#[put("/profiles/<id>")]
pub fn update(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "update profile"
}

#[delete("/profiles/<id>")]
pub fn delete(_conn: Db, id: i32) -> &'static str {
    println!("{}", id);
    "delete profile"
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
