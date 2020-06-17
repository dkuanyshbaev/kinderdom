use super::schema::profiles;
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

const PROFILES_PER_PAGE: i64 = 9;

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "profiles"]
pub struct NewProfile {
    pub name: String,
    pub photo: String,
    pub description: String,
    pub published: bool,
    pub en: bool,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub photo: String,
    pub description: String,
    pub published: bool,
    pub en: bool,
    pub created_at: NaiveDateTime,
}

impl Profile {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Profile>> {
        profiles::table.order(profiles::id.desc()).load(connection)
    }

    pub fn published(connection: &PgConnection, page: i64) -> QueryResult<Vec<Profile>> {
        profiles::table
            .filter(profiles::published.eq(true))
            .offset(page * PROFILES_PER_PAGE)
            .limit(PROFILES_PER_PAGE)
            .order(profiles::id.desc())
            .load(connection)
    }

    pub fn pages_total(connection: &PgConnection) -> usize {
        let mut total = 0;
        if let Ok(ps) = profiles::table
            .filter(profiles::published.eq(true))
            .load::<Profile>(connection)
        {
            total = ps.len() / PROFILES_PER_PAGE as usize + 1;
        }
        total
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
        let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
            MultipartFormDataField::file("photo"),
            MultipartFormDataField::text("name"),
            MultipartFormDataField::text("description"),
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

        let mut photo = "".to_string();
        if let Some(file_fields) = multipart_form.files.get("photo") {
            let file_field = &file_fields[0];
            let file_name = &file_field.file_name;
            let path = &file_field.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    photo = uuid_file_name(file_path);
                    save_file(path, &photo);
                }
            }
        }

        let mut name = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("name") {
            let text_field = text_fields.remove(0);
            name = text_field.text;
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

        let mut published = false;
        if let Some(mut text_fields) = multipart_form.texts.remove("published") {
            let text_field = text_fields.remove(0);
            if text_field.text == "on" {
                published = true;
            }
        }

        Success(NewProfile {
            name,
            photo,
            description,
            en,
            published,
        })
    }
}
