use super::schema::reports;
use super::utils::{delete_file, file_name_with_prefix, save_file};
use crate::errors::KinderError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    FileField, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, TextField,
};

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "reports"]
pub struct NewReport {
    pub pdf: String,
    pub description: String,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Report {
    pub id: i32,
    pub pdf: String,
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
        mut new_report: NewReport,
        id: i32,
    ) -> QueryResult<Report> {
        let old_report: Report = Self::get(connection, id)?;
        if new_report.pdf == "".to_string() {
            // keep old file in case of update without file
            new_report.pdf = old_report.pdf.clone();
        } else {
            delete_file(&old_report.pdf);
        }

        diesel::update(&old_report)
            .set(new_report)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Report> {
        // remove related file
        let report: Report = Self::get(connection, id)?;
        delete_file(&report.pdf);

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
            .push(MultipartFormDataField::file("pdf"));
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

        let mut pdf = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("pdf") {
            let file_name = &file.file_name;
            let path = &file.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    pdf = file_name_with_prefix(file_path);
                    save_file(path, &pdf);
                }
            }
        }

        let mut description = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("description") {
            description = &text.text;
        }

        Success(NewReport {
            pdf,
            description: description.to_string(),
        })
    }
}
