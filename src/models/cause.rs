use super::schema::causes;
use super::utils::{delete_file, save_file, uuid_file_name};
use crate::errors::KinderError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "causes"]
pub struct NewCause {
    pub name: String,
    pub image: String,
    pub needed: i32,
    pub collected: i32,
    pub location: String,
    pub organisation: String,
    pub description: String,
    pub vital: bool,
    pub published: bool,
    pub en: bool,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Cause {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub needed: i32,
    pub collected: i32,
    pub location: String,
    pub organisation: String,
    pub description: String,
    pub vital: bool,
    pub published: bool,
    pub en: bool,
    pub created_at: NaiveDateTime,
}

impl Cause {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Cause>> {
        causes::table.order(causes::id.desc()).load(connection)
    }

    pub fn published(connection: &PgConnection) -> QueryResult<Vec<Cause>> {
        causes::table
            .filter(causes::published.eq(true))
            .limit(4)
            .order(causes::id.desc())
            .load(connection)
    }

    pub fn vital(connection: &PgConnection) -> QueryResult<Vec<Cause>> {
        causes::table
            .filter(causes::published.eq(true))
            .filter(causes::vital.eq(true))
            .limit(3)
            .order(causes::id.desc())
            .load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Cause> {
        causes::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_cause: NewCause) -> QueryResult<Cause> {
        diesel::insert_into(causes::table)
            .values(new_cause)
            .get_result(connection)
    }

    pub fn update(
        connection: &PgConnection,
        mut new_cause: NewCause,
        id: i32,
    ) -> QueryResult<Cause> {
        let old_cause: Cause = Self::get(connection, id)?;
        if new_cause.image == "".to_string() {
            // keep old image name in case of update without image
            new_cause.image = old_cause.image.clone();
        } else {
            delete_file(&old_cause.image);
        }

        diesel::update(&old_cause)
            .set(new_cause)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Cause> {
        // remove image
        let cause: Cause = Self::get(connection, id)?;
        delete_file(&cause.image);

        diesel::delete(&cause).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewCause {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
            MultipartFormDataField::file("image"),
            MultipartFormDataField::text("name"),
            MultipartFormDataField::text("need"),
            MultipartFormDataField::text("collected"),
            MultipartFormDataField::text("location"),
            MultipartFormDataField::text("organisation"),
            MultipartFormDataField::text("description"),
            MultipartFormDataField::text("vital"),
            MultipartFormDataField::text("en"),
            MultipartFormDataField::text("published"),
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

        let mut image = "".to_string();
        if let Some(file_fields) = multipart_form.files.get("image") {
            let file_field = &file_fields[0];
            let file_name = &file_field.file_name;
            let path = &file_field.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    image = uuid_file_name(file_path);
                    save_file(path, &image);
                }
            }
        }

        let mut name = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("name") {
            let text_field = text_fields.remove(0);
            name = text_field.text;
        }

        let mut needed = 0;
        if let Some(mut text_fields) = multipart_form.texts.remove("needed") {
            let text_field = text_fields.remove(0);
            let amount = text_field.text;
            needed = amount.parse().unwrap();
        }

        let mut collected = 0;
        if let Some(mut text_fields) = multipart_form.texts.remove("collected") {
            let text_field = text_fields.remove(0);
            let amount = text_field.text;
            collected = amount.parse().unwrap();
        }

        let mut location = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("location") {
            let text_field = text_fields.remove(0);
            location = text_field.text;
        }

        let mut organisation = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("organisation") {
            let text_field = text_fields.remove(0);
            organisation = text_field.text;
        }

        let mut description = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("description") {
            let text_field = text_fields.remove(0);
            description = text_field.text;
        }

        let mut vital = false;
        if let Some(mut text_fields) = multipart_form.texts.remove("vital") {
            let text_field = text_fields.remove(0);
            if text_field.text == "on" {
                vital = true;
            }
        }

        let mut en = false;
        if let Some(mut text_fields) = multipart_form.texts.remove("en") {
            let text_field = text_fields.remove(0);
            if text_field.text == "on" {
                en = true;
            }
        }

        let mut published = false;
        if let Some(mut text_fields) = multipart_form.texts.remove("published") {
            let text_field = text_fields.remove(0);
            if text_field.text == "on" {
                published = true;
            }
        }

        Success(NewCause {
            name,
            image,
            needed,
            collected,
            location,
            organisation,
            description,
            vital,
            en,
            published,
        })
    }
}
