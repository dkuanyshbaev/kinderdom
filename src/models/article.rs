use super::schema::articles;
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
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub image: String,
    pub content: String,
    pub welfare: bool,
    pub published: bool,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub image: String,
    pub content: String,
    pub welfare: bool,
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
        let old_article: Article = Self::get(connection, id)?;
        if new_article.image == "".to_string() {
            // keep old image name in case of update without image
            new_article.image = old_article.image.clone();
        } else {
            delete_file(&old_article.image);
        }

        diesel::update(&old_article)
            .set(new_article)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Article> {
        // remove related image
        let article: Article = Self::get(connection, id)?;
        delete_file(&article.image);

        diesel::delete(&article).get_result(connection)
    }
}

// we need this custom impl for multipart form
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
            .push(MultipartFormDataField::text("welfare"));
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

        let mut image = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("image") {
            let file_name = &file.file_name;
            let path = &file.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    image = file_name_with_prefix(file_path);
                    save_file(path, &image);
                }
            }
        }

        let mut title = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("title") {
            title = &text.text;
        }

        let mut content = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("content") {
            content = &text.text;
        }

        let mut welfare = false;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("welfare") {
            if &text.text == "on" {
                welfare = true;
            }
        }

        let mut published = false;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("published") {
            if &text.text == "on" {
                published = true;
            }
        }

        Success(NewArticle {
            title: title.to_string(),
            image,
            content: content.to_string(),
            welfare,
            published,
        })
    }
}
