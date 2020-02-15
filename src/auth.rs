use crate::rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};

// #[derive(Debug)]
// pub struct Admin(String);
//
// impl<'a, 'r> FromRequest<'a, 'r> for Admin {
//     type Error = ();
//
//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, ()> {
//         // This will unconditionally query the database!
//         // let user = request.guard::<User>()?;
//
//         if user.is_admin {
//             Outcome::Success(Admin { "test".to_string() })
//         } else {
//             Outcome::Forward(())
//         }
//     }
// }

#[derive(FromForm)]
pub struct LoginForm {
    // username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Admin(usize);

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, !> {
        request
            .cookies()
            .get_private("admin_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| Admin(id))
            .or_forward(())
    }
}

// impl<'a, 'r> FromRequest<'a, 'r> for HeaderCount {
//     type Error = !;
//
//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, !> {
//         Success(HeaderCount(request.headers().len()))
//     }
// }

//--------------------------------------------------------------------------
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
//--------------------------------------------------------------------------

// #[cfg(test)]
// mod test {
//     use rocket::http::Header;
//     use rocket::local::Client;
//
//     fn test_header_count<'h>(headers: Vec<Header<'static>>) {
//         let client = Client::new(super::rocket()).unwrap();
//         let mut req = client.get("/");
//         for header in headers.iter().cloned() {
//             req.add_header(header);
//         }
//
//         let mut response = req.dispatch();
//         let expect = format!("Your request contained {} headers!", headers.len());
//         assert_eq!(response.body_string(), Some(expect));
//     }
//
//     #[test]
//     fn test_n_headers() {
//         for i in 0..50 {
//             let headers = (0..i)
//                 .map(|n| Header::new(n.to_string(), n.to_string()))
//                 .collect();
//
//             test_header_count(headers);
//         }
//     }
// }
