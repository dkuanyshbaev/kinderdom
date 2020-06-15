use super::schema::causes;
use super::utils::{delete_file, save_file, uuid_file_name};
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
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::file("image"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("name"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("needed"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("collected"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("location"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("organisation"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("description"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("vital"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("published"));

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

        let mut image = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("image") {
            let file_name = &file.file_name;
            let path = &file.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    image = uuid_file_name(file_path);
                    save_file(path, &image);
                }
            }
        }

        let mut name = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("name") {
            name = &text.text;
        }

        let mut needed = 0;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("needed") {
            let amount = &text.text;
            needed = amount.parse().unwrap();
        }

        let mut collected = 0;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("collected") {
            let amount = &text.text;
            collected = amount.parse().unwrap();
        }

        let mut location = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("location") {
            location = &text.text;
        }

        let mut organisation = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("organisation") {
            organisation = &text.text;
        }

        let mut description = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("description") {
            description = &text.text;
        }

        let mut vital = false;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("vital") {
            if &text.text == "on" {
                vital = true;
            }
        }

        let mut published = false;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("published") {
            if &text.text == "on" {
                published = true;
            }
        }

        Success(NewCause {
            name: name.to_string(),
            image,
            needed,
            collected,
            location: location.to_string(),
            organisation: organisation.to_string(),
            description: description.to_string(),
            vital,
            published,
        })
    }
}
