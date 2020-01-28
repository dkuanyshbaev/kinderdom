extern crate rocket_contrib;
extern crate serde_json;

// use super::Config;
use rocket::Request;
// use rocket::State;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct TemplateContext {
    pub name: String,
    pub items: Vec<&'static str>,
}

// #[get("/")]
// // pub fn index(config: State<Config>) -> Template {
// pub fn index() -> Template {
//     // let secret = &config.secret;
//
//     // println!("------->>>>>> {}", secret);
//
//     let name = "Denis".to_string();
//     let context = TemplateContext {
//         name,
//         items: vec!["One", "Two", "Three"],
//     };
//     Template::render("index", &context)
// }

#[get("/about")]
pub fn about() -> &'static str {
    "about page"
}

#[get("/profiles")]
pub fn profiles() -> &'static str {
    "add profiles"
}

#[get("/news")]
pub fn news() -> &'static str {
    "add news"
}

// #[get("/projects")]
// pub fn projects(user: &Admin) -> Projects {
//     "projects"
// }

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

//--------------------------------------------------------------------------

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
