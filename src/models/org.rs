use super::schema::orgs;
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
#[table_name = "orgs"]
pub struct NewOrg {
    pub name: String,
    pub logo: String,
    pub description: String,
    pub published: bool,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Org {
    pub id: i32,
    pub name: String,
    pub logo: String,
    pub description: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

impl Org {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Org>> {
        orgs::table.order(orgs::id.desc()).load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Org> {
        orgs::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_org: NewOrg) -> QueryResult<Org> {
        diesel::insert_into(orgs::table)
            .values(new_org)
            .get_result(connection)
    }

    pub fn update(connection: &PgConnection, mut new_org: NewOrg, id: i32) -> QueryResult<Org> {
        let old_org: Org = Self::get(connection, id)?;
        if new_org.logo == "".to_string() {
            // keep old logo name in case of update without logo
            new_org.logo = old_org.logo.clone();
        } else {
            delete_file(&old_org.logo);
        }

        diesel::update(&old_org).set(new_org).get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Org> {
        // remove related photo
        let org: Org = Self::get(connection, id)?;
        delete_file(&org.logo);

        diesel::delete(&org).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewOrg {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::file("logo"));
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

        let mut logo = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("logo") {
            let file_name = &file.file_name;
            let path = &file.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    logo = file_name_with_prefix(file_path);
                    save_file(path, &logo);
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

        Success(NewOrg {
            name: name.to_string(),
            logo,
            description: description.to_string(),
            published,
        })
    }
}
