use super::schema::profiles;
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
#[table_name = "profiles"]
pub struct NewProfile {
    pub name: String,
    pub photo: String,
    pub description: String,
    pub published: bool,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub photo: String,
    pub description: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

impl Profile {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Profile>> {
        profiles::table.order(profiles::id.desc()).load(connection)
    }

    pub fn published(connection: &PgConnection) -> QueryResult<Vec<Profile>> {
        profiles::table
            .filter(profiles::published.eq(true))
            .limit(4)
            .order(profiles::id.desc())
            .load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Profile> {
        profiles::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_profile: NewProfile) -> QueryResult<Profile> {
        diesel::insert_into(profiles::table)
            .values(new_profile)
            .get_result(connection)
    }

    pub fn update(
        connection: &PgConnection,
        mut new_profile: NewProfile,
        id: i32,
    ) -> QueryResult<Profile> {
        let old_profile: Profile = Self::get(connection, id)?;
        if new_profile.photo == "".to_string() {
            // keep old image name in case of update without image
            new_profile.photo = old_profile.photo.clone();
        } else {
            delete_file(&old_profile.photo);
        }

        diesel::update(&old_profile)
            .set(new_profile)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Profile> {
        // remove photo
        let profile: Profile = Self::get(connection, id)?;
        delete_file(&profile.photo);

        diesel::delete(&profile).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewProfile {
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

        let mut photo = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("photo") {
            let file_name = &file.file_name;
            let path = &file.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    photo = file_name_with_prefix(file_path);
                    save_file(path, &photo);
                }
            }
        }

        let mut name = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("name") {
            name = &text.text;
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

        Success(NewProfile {
            name: name.to_string(),
            photo,
            description: description.to_string(),
            published,
        })
    }
}
