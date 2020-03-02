use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;
use std::convert::From;
use std::{error, fmt};

#[derive(Debug)]
pub enum KinderError {
    NotFound,
    InternalServerError,
    BadRequest,
}

impl fmt::Display for KinderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KinderError::NotFound => write!(f, "NotFound"),
            KinderError::InternalServerError => write!(f, "InternalServerError"),
            KinderError::BadRequest => write!(f, "BadRequest"),
        }
    }
}

impl error::Error for KinderError {
    fn description(&self) -> &str {
        match *self {
            KinderError::NotFound => "Record not found",
            KinderError::InternalServerError => "Internal server error",
            KinderError::BadRequest => "Bad Request",
        }
    }
}

impl From<DieselError> for KinderError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => KinderError::NotFound,
            _ => KinderError::InternalServerError,
        }
    }
}

impl<'r> Responder<'r> for KinderError {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'r> {
        match self {
            KinderError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}
