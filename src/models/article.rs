use super::schema::articles;
use crate::errors::KinderError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    mime, FileField, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, TextField,
};

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub image: String,
    pub content: String,
    pub published: bool,
}

#[derive(Serialize, Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub image: String,
    pub content: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

impl Article {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Article>> {
        articles::table.order(articles::id.desc()).load(connection)
    }

    pub fn published(connection: &PgConnection) -> QueryResult<Vec<Article>> {
        articles::table
            .filter(articles::published.eq(true))
            .limit(4)
            .order(articles::id.desc())
            .load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Article> {
        articles::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, article: NewArticle) -> QueryResult<Article> {
        diesel::insert_into(articles::table)
            .values(article)
            .get_result(connection)
    }

    pub fn update(connection: &PgConnection, article: NewArticle, id: i32) -> QueryResult<Article> {
        diesel::update(articles::table.find(id))
            .set(article)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Article> {
        diesel::delete(articles::table.find(id)).get_result(connection)
    }
}

impl FromDataSimple for NewArticle {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut options = MultipartFormDataOptions::new();

        options.allowed_fields.push(
            MultipartFormDataField::text("title")
                .content_type_by_string(Some(mime::STAR_STAR))
                .unwrap(),
        );
        options.allowed_fields.push(
            MultipartFormDataField::file("image")
                // .size_limit(32 * 1024 * 1024)
                .content_type_by_string(Some(mime::IMAGE_STAR))
                .unwrap(),
        );
        options.allowed_fields.push(
            MultipartFormDataField::text("content")
                .content_type_by_string(Some(mime::STAR_STAR))
                .unwrap(),
        );
        options.allowed_fields.push(
            MultipartFormDataField::text("published")
                .content_type_by_string(Some(mime::STAR_STAR))
                .unwrap(),
        );

        // check if the content type is set properly
        let content_type = match request.content_type() {
            Some(content_type) => content_type,
            _ => {
                return Failure((Status::BadRequest, KinderError::InternalServerError));
            }
        };

        // do the form parsing and return on error
        let multipart_form = match MultipartFormData::parse(&content_type, data, options) {
            Ok(m) => m,
            Err(_) => {
                return Failure((Status::BadRequest, KinderError::InternalServerError));
            }
        };

        // let title = multipart_form.texts.get("title");
        if let Some(title) = multipart_form.texts.get("title") {
            match title {
                TextField::Single(text) => {
                    let _content_type = &text.content_type;
                    let _file_name = &text.file_name;
                    let _text = &text.text;
                    // You can now deal with the raw data.
                    println!("-------- title: {:?}", title)
                }
                TextField::Multiple(_texts) => {
                    // Because we only put one "text" field to the allowed_fields, this arm will not be matched.
                    return Failure((
                        Status::BadRequest,
                        // MultipartError::new(format!("Extra text fields supplied")),
                        KinderError::InternalServerError,
                    ));
                }
            }
        }

        let image = multipart_form.files.get("image");
        if let Some(image) = image {
            match image {
                FileField::Single(file) => {
                    let _content_type = &file.content_type;
                    let file_name = &file.file_name;
                    let path = &file.path;
                    // You can now deal with the uploaded file.
                    // The file will be deleted automatically when the MultipartFormData instance is dropped.
                    // If you want to handle that file by your own, instead of killing it,
                    // just remove it out from the MultipartFormData instance.
                    // let image = multipart_form_data.files.remove("image");
                    println!("-------- we are here - can save image. {:?}", image);

                    // std::fs::copy("foo.txt", "bar.txt")?;
                    if let Some(file_path) = file_name {
                        match std::fs::copy(path, format!("static/upload/{}", file_path)) {
                            Ok(_) => (),
                            Err(e) => println!("{:?}", e),
                        }
                    }
                }
                FileField::Multiple(_files) => {
                    // Because we only put one "photo" field to the allowed_fields, this arm will not be matched.
                }
            }
        }

        let content = multipart_form.texts.get("content");
        if let Some(content) = content {
            match content {
                TextField::Single(text) => {
                    let _content_type = &text.content_type;
                    let _file_name = &text.file_name;
                    let _text = &text.text;
                    // You can now deal with the raw data.
                    println!("-------- content: {:?}", content)
                }
                TextField::Multiple(_texts) => {
                    // Because we only put one "text" field to the allowed_fields, this arm will not be matched.
                }
            }
        }

        let published = multipart_form.texts.get("published");
        if let Some(published) = published {
            match published {
                TextField::Single(text) => {
                    let _content_type = &text.content_type;
                    let _file_name = &text.file_name;
                    let _text = &text.text;
                    // You can now deal with the raw data.
                    println!("-------- published: {:?}", published)
                }
                TextField::Multiple(_bytes) => {
                    // Because we only put one "fingerprint" field to the allowed_fields, this arm will not be matched.
                }
            }
        }

        // check if the form has the avatar image
        // let image_part: &RawField = match multipart_form.raw.get("avatar") {
        //     Some(image_part) => image_part,
        //     _ => {
        //         return Failure((
        //             Status::BadRequest,
        //             MultipartError::new(format!("Missing field 'avatar'")),
        //         ))
        //     }
        // };

        // verify only the data we want is being passed, one text field and one binary
        // match post_json_part {
        //     TextField::Single(text) => {
        //         let json_string = &text.text.replace('\'', "\"");
        //         post_obj = match serde_json::from_str::<User>(json_string) {
        //             Ok(insert) => insert,
        //             Err(e) => {
        //                 return Failure((
        //                     Status::BadRequest,
        //                     MultipartError::new(format!("{:?}", e)),
        //                 ))
        //             }
        //         };
        //     }
        //     TextField::Multiple(_text) => {
        //         return Failure((
        //             Status::BadRequest,
        //             MultipartError::new(format!("Extra text fields supplied")),
        //         ))
        //     }
        // };

        // match image_part {
        //     RawField::Single(raw) => {
        //         image_bytes = raw.raw.clone();
        //     }
        //     RawField::Multiple(_raw) => {
        //         return Failure((
        //             Status::BadRequest,
        //             MultipartError::new(format!("Extra image fields supplied")),
        //         ))
        //     }
        // };

        Success(NewArticle {
            title: "".to_string(),
            image: "".to_string(),
            content: "".to_string(),
            published: false,
        })
    }
}
