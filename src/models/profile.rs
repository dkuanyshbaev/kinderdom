use super::schema::profiles;
use crate::errors::KinderError;
use chrono::{Datelike, NaiveDateTime, Utc};
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
    pub video: String,
    pub description: String,
    pub published: bool,
}

#[derive(Serialize, Queryable, Debug)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub photo: String,
    pub video: String,
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
        let old_profile: Profile = profiles::table.find(id).get_result(connection)?;
        if new_profile.photo == "".to_string() {
            // we need to keep old photo name in case of update without photo
            new_profile.photo = old_profile.photo;
        } else {
            // remove old photo in case of update with photo
            if let Err(error) = std::fs::remove_file(format!("static/upload/{}", old_profile.photo))
            {
                println!("File error: {}", error);
            }
        }

        diesel::update(profiles::table.find(id))
            .set(new_profile)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Profile> {
        // remove photo
        let profile: Profile = profiles::table.find(id).get_result(connection)?;
        if let Err(error) = std::fs::remove_file(format!("static/upload/{}", profile.photo)) {
            println!("File error: {}", error);
        }

        diesel::delete(profiles::table.find(id)).get_result(connection)
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
            .push(MultipartFormDataField::text("video"));
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

        let mut new_name = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("name") {
            new_name = &text.text;
        }

        let mut new_photo = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("photo") {
            let file_name = &file.file_name;
            let path = &file.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    // build "unique" filename with current date prefix
                    let now = Utc::now();
                    let (_, year) = now.year_ce();
                    let file_name = format!("{}_{}_{}_{}", year, now.month(), now.day(), file_path);

                    // copy file from tmp with new filename
                    match std::fs::copy(path, format!("static/upload/{}", file_name)) {
                        Ok(_) => {
                            new_photo = file_name;
                        }
                        Err(e) => println!("File error: {:?}", e),
                    }
                }
            }
        }

        let mut new_video = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("video") {
            new_video = &text.text;
        }

        let mut new_description = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("description") {
            new_description = &text.text;
        }

        let mut new_published_value = false;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("published") {
            if &text.text == "on" {
                new_published_value = true;
            }
        }

        Success(NewProfile {
            name: new_name.to_string(),
            photo: new_photo,
            video: new_video.to_string(),
            description: new_description.to_string(),
            published: new_published_value,
        })
    }
}
