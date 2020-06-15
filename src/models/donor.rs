use super::schema::donors;
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
#[table_name = "donors"]
pub struct NewDonor {
    pub name: String,
    pub photo: String,
    pub position: String,
    pub quote: String,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Donor {
    pub id: i32,
    pub name: String,
    pub photo: String,
    pub position: String,
    pub quote: String,
    pub created_at: NaiveDateTime,
}

impl Donor {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Donor>> {
        donors::table.order(donors::id.desc()).load(connection)
    }

    pub fn last(connection: &PgConnection) -> QueryResult<Vec<Donor>> {
        donors::table
            .limit(4)
            .order(donors::id.desc())
            .load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Donor> {
        donors::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_donor: NewDonor) -> QueryResult<Donor> {
        diesel::insert_into(donors::table)
            .values(new_donor)
            .get_result(connection)
    }

    pub fn update(
        connection: &PgConnection,
        mut new_donor: NewDonor,
        id: i32,
    ) -> QueryResult<Donor> {
        let old_donor: Donor = Self::get(connection, id)?;
        if new_donor.photo == "".to_string() {
            // keep old photo name in case of update without photo
            new_donor.photo = old_donor.photo.clone();
        } else {
            delete_file(&old_donor.photo);
        }

        diesel::update(&old_donor)
            .set(new_donor)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Donor> {
        // remove related photo
        let donor: Donor = Self::get(connection, id)?;
        delete_file(&donor.photo);

        diesel::delete(&donor).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewDonor {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::file("photo"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("name"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("position"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("quote"));

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

        let mut photo = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("photo") {
            let file_name = &file.file_name;
            let path = &file.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    photo = uuid_file_name(file_path);
                    save_file(path, &photo);
                }
            }
        }

        let mut name = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("name") {
            name = &text.text;
        }

        let mut position = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("position") {
            position = &text.text;
        }

        let mut quote = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("quote") {
            quote = &text.text;
        }

        Success(NewDonor {
            name: name.to_string(),
            photo,
            position: position.to_string(),
            quote: quote.to_string(),
        })
    }
}
