use super::schema::cats;
// use crate::errors::KinderError;
// use chrono::NaiveDateTime;
use diesel::prelude::*;
// use rocket::data::{FromDataSimple, Outcome};
// use rocket::http::Status;
// use rocket::{Data, Outcome::*, Request};
// use rocket_multipart_form_data::{
//     FileField, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, TextField,
// };

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "cats"]
pub struct NewCat {
    pub name: String,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Cat {
    pub id: i32,
    pub name: String,
}

impl Cat {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Cat>> {
        cats::table.order(cats::id.asc()).load(connection)
    }

    // pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Report> {
    //     reports::table.find(id).get_result(connection)
    // }
    //
    // pub fn insert(connection: &PgConnection, new_report: NewReport) -> QueryResult<Report> {
    //     diesel::insert_into(reports::table)
    //         .values(new_report)
    //         .get_result(connection)
    // }
    //
    // pub fn update(
    //     connection: &PgConnection,
    //     mut new_report: NewReport,
    //     id: i32,
    // ) -> QueryResult<Report> {
    //     let old_report: Report = Self::get(connection, id)?;
    //     if new_report.pdf == "".to_string() {
    //         // keep old file in case of update without file
    //         new_report.pdf = old_report.pdf.clone();
    //     } else {
    //         delete_file(&old_report.pdf);
    //     }
    //
    //     diesel::update(&old_report)
    //         .set(new_report)
    //         .get_result(connection)
    // }
    //
    // pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Report> {
    //     // remove related file
    //     let report: Report = Self::get(connection, id)?;
    //     delete_file(&report.pdf);
    //
    //     diesel::delete(&report).get_result(connection)
    // }
}
