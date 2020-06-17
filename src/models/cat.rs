use super::schema::cats;
use crate::errors::KinderError;
use diesel::prelude::*;
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "cats"]
pub struct NewCat {
    pub name: String,
    pub en: bool,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub en: bool,
}

impl Cat {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Cat>> {
        cats::table.order(cats::id.asc()).load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Cat> {
        cats::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_cat: NewCat) -> QueryResult<Cat> {
        diesel::insert_into(cats::table)
            .values(new_cat)
            .get_result(connection)
    }

    pub fn update(connection: &PgConnection, new_cat: NewCat, id: i32) -> QueryResult<Cat> {
        let old_cat: Cat = Self::get(connection, id)?;
        diesel::update(&old_cat).set(new_cat).get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Cat> {
        let cat: Cat = Self::get(connection, id)?;
        diesel::delete(&cat).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewCat {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
            MultipartFormDataField::text("name"),
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

        let mut name = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("name") {
            let text_field = text_fields.remove(0);
            name = text_field.text;
        }

        let mut en = false;
        if let Some(mut text_fields) = multipart_form.texts.remove("en") {
            let text_field = text_fields.remove(0);
            if text_field.text == "on" {
                en = true;
            }
        }

        Success(NewCat { name, en })
    }
}
