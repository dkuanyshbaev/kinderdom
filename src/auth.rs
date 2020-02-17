use crate::rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};

#[derive(FromForm)]
pub struct LoginForm {
    pub password: String,
}

#[derive(Debug)]
pub struct Admin(usize);

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, !> {
        request
            .cookies()
            .get_private("admin")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| Admin(id))
            .or_forward(())
    }
}
