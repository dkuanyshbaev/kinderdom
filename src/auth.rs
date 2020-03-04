use crate::errors::KinderError;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

#[derive(FromForm)]
pub struct LoginForm {
    pub password: String,
}

#[derive(Debug)]
pub struct Admin(usize);

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = KinderError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, KinderError> {
        match request
            .cookies()
            .get_private("admin")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| Admin(id))
        {
            Some(id) => Outcome::Success(id),
            None => Outcome::Failure((Status::Unauthorized, KinderError::Unauthorized)),
        }
    }
}
