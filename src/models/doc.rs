use super::schema::docs;
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
#[table_name = "docs"]
pub struct NewDoc {
    pub pdf: String,
    pub description: String,
    pub published: bool,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Doc {
    pub id: i32,
    pub pdf: String,
    pub description: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

impl Doc {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Doc>> {
        docs::table.order(docs::id.desc()).load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Doc> {
        docs::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_doc: NewDoc) -> QueryResult<Doc> {
        diesel::insert_into(docs::table)
            .values(new_doc)
            .get_result(connection)
    }

    pub fn update(connection: &PgConnection, mut new_doc: NewDoc, id: i32) -> QueryResult<Doc> {
        let old_doc: Doc = Self::get(connection, id)?;
        if new_doc.pdf == "".to_string() {
            // keep old file in case of update without file
            new_doc.pdf = old_doc.pdf.clone();
        } else {
            delete_file(&old_doc.pdf);
        }

        diesel::update(&old_doc).set(new_doc).get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Doc> {
        // remove related file
        let doc: Doc = Self::get(connection, id)?;
        delete_file(&doc.pdf);

        diesel::delete(&doc).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewDoc {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::file("file_name"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("description"));
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

        let mut pdf = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("file_name") {
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

        let mut published = false;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("published") {
            if &text.text == "on" {
                published = true;
            }
        }

        Success(NewDoc {
            pdf,
            description: description.to_string(),
            published,
        })
    }
}
