use super::schema::cats;
use crate::errors::KinderError;
use diesel::prelude::*;
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, TextField,
};

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
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::text("name"));

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

        let mut name = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("name") {
            name = &text.text;
        }

        Success(NewCat {
            name: name.to_string(),
        })
    }
}
