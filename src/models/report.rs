use super::schema::reports;
use crate::errors::KinderError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, TextField,
};

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "reports"]
pub struct NewReport {
    pub url: String,
    pub description: String,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Report {
    pub id: i32,
    pub url: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}

impl Report {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Report>> {
        reports::table.order(reports::id.desc()).load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Report> {
        reports::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_report: NewReport) -> QueryResult<Report> {
        diesel::insert_into(reports::table)
            .values(new_report)
            .get_result(connection)
    }

    pub fn update(
        connection: &PgConnection,
        new_report: NewReport,
        id: i32,
    ) -> QueryResult<Report> {
        let old_report: Report = Self::get(connection, id)?;
        diesel::update(&old_report)
            .set(new_report)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Report> {
        let report: Report = Self::get(connection, id)?;
        diesel::delete(&report).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewReport {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::text("url"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("description"));

        // check if the content type is set properly
        let content_type = match request.content_type() {
            Some(content_type) => content_type,
            _ => {
                return Failure((Status::BadRequest, KinderError::BadRequest));
            }
        };

        // do the form parsing and return on error
        let multipart_form = match MultipartFormData::parse(&content_type, data, options) {
            Ok(multipart) => multipart,
            Err(error) => {
                println!("Multipart form parsing error: {:?}", error);
                return Failure((Status::BadRequest, KinderError::BadRequest));
            }
        };

        let mut url = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("url") {
            url = &text.text;
        }

        let mut description = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("description") {
            description = &text.text;
        }

        Success(NewReport {
            url: url.to_string(),
            description: description.to_string(),
        })
    }
}
