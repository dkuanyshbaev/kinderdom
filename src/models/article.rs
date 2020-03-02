use super::schema::articles;
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

    pub fn insert(connection: &PgConnection, new_article: NewArticle) -> QueryResult<Article> {
        diesel::insert_into(articles::table)
            .values(new_article)
            .get_result(connection)
    }

    pub fn update(
        connection: &PgConnection,
        mut new_article: NewArticle,
        id: i32,
    ) -> QueryResult<Article> {
        let old_article: Article = articles::table.find(id).get_result(connection)?;

        // we need to keep old image name in case of update without image
        if new_article.image == "".to_string() {
            new_article.image = old_article.image;
        } else {
            if let Err(error) = std::fs::remove_file(format!("static/upload/{}", old_article.image))
            {
                println!("File error: {}", error);
            }
        }

        diesel::update(articles::table.find(id))
            .set(new_article)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Article> {
        let article: Article = articles::table.find(id).get_result(connection)?;
        if let Err(error) = std::fs::remove_file(format!("static/upload/{}", article.image)) {
            println!("File error: {}", error);
        }

        diesel::delete(articles::table.find(id)).get_result(connection)
    }
}

impl FromDataSimple for NewArticle {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::file("image"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("title"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("content"));
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

        let mut new_title = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("title") {
            new_title = &text.text;
        }

        let mut new_image = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("image") {
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
                            new_image = file_name;
                        }
                        Err(e) => println!("File error: {:?}", e),
                    }
                }
            }
        }

        let mut new_content = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("content") {
            new_content = &text.text;
        }

        let mut new_published_value = false;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("published") {
            if &text.text == "on" {
                new_published_value = true;
            }
        }

        Success(NewArticle {
            title: new_title.to_string(),
            image: new_image,
            content: new_content.to_string(),
            published: new_published_value,
        })
    }
}
