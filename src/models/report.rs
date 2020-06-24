use super::schema::reports;
use crate::errors::KinderError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

const REPORTS_PER_PAGE: i64 = 9;

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "reports"]
pub struct NewReport {
    pub url: String,
    pub description: String,
    pub en: bool,
}

#[derive(Serialize, Queryable, Identifiable, Clone, Debug)]
pub struct Report {
    pub id: i32,
    pub url: String,
    pub description: String,
    pub en: bool,
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

    pub fn paginated(
        connection: &PgConnection,
        page: Option<u8>,
    ) -> QueryResult<(u8, u8, Vec<Report>)> {
        let mut page_num = 0;
        if let Some(p) = page {
            page_num = p;
        }

        match Self::all(connection) {
            Ok(reports) => {
                let max = reports.len();
                let total = (max as i64 / REPORTS_PER_PAGE + 1) as u8;

                let mut offset = page_num as usize * REPORTS_PER_PAGE as usize;
                if offset > max {
                    offset = max;
                }

                let mut limit = offset + REPORTS_PER_PAGE as usize;
                if limit > max {
                    limit = max;
                }

                let reports = reports[offset..limit].to_vec();

                Ok((total, page_num, reports))
            }
            Err(error) => Err(error),
        }
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewReport {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
            MultipartFormDataField::text("url"),
            MultipartFormDataField::text("description"),
            MultipartFormDataField::text("en"),
        ]);

        // check if the content type is set properly
        let content_type = match request.content_type() {
            Some(content_type) => content_type,
            _ => {
                return Failure((Status::BadRequest, KinderError::BadRequest));
            }
        };

        // do the form parsing and return on error
        let mut multipart_form = match MultipartFormData::parse(&content_type, data, options) {
            Ok(multipart) => multipart,
            Err(error) => {
                println!("Multipart form parsing error: {:?}", error);
                return Failure((Status::BadRequest, KinderError::BadRequest));
            }
        };

        let mut url = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("url") {
            let text_field = text_fields.remove(0);
            url = text_field.text;
        }

        let mut description = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("description") {
            let text_field = text_fields.remove(0);
            description = text_field.text;
        }

        let mut en = false;
        if let Some(mut text_fields) = multipart_form.texts.remove("en") {
            let text_field = text_fields.remove(0);
            if text_field.text == "on" {
                en = true;
            }
        }

        Success(NewReport {
            url,
            description,
            en,
        })
    }
}
