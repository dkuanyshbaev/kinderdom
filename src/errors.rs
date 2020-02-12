use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotFound,
    InternalServerError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound => f.write_str("NotFound"),
            Error::InternalServerError => f.write_str("InternalServerError"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotFound => "Record not found",
            Error::InternalServerError => "Internal server error",
        }
    }
}

impl From<DieselError> for Error {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => Error::NotFound,
            _ => Error::InternalServerError,
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'r> {
        match self {
            Error::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}

// impl<'r> Responder<'r> for Profile {
//     fn respond_to(self, _: &Request) -> response::Result<'r> {
//         // match self {
//         //     Some(profile) => println!("!!!!!!!!!!!!!!"),
//         //     None => println!("&&&&&&&&&&&&&&&&&&"),
//         // }
//         Ok(Template::render("admin/profiles/show", self))
//
//         // match self {
//         //     Err(_) => Err(status::NotFound),
//         //     Ok(profile) => Ok(Template::render("admin/profiles/show", profile)),
//         // }
//
//         // Response::build()
//         //     .sized_body(Cursor::new(format!("{}:{}", self.name, self.age)))
//         //     .raw_header("X-Person-Name", self.name)
//         //     .raw_header("X-Person-Age", self.age.to_string())
//         //     .header(ContentType::new("application", "x-person"))
//         //     .ok()
//     }
// }
