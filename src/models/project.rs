use super::schema::projects;
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
#[table_name = "projects"]
pub struct NewProject {
    pub name: String,
    pub image: String,
    pub needed: i32,
    pub collected: i32,
    pub description: String,
    pub published: bool,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub needed: i32,
    pub collected: i32,
    pub description: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

impl Project {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Project>> {
        projects::table.order(projects::id.desc()).load(connection)
    }

    pub fn published(connection: &PgConnection) -> QueryResult<Vec<Project>> {
        projects::table
            .filter(projects::published.eq(true))
            .limit(4)
            .order(projects::id.desc())
            .load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Project> {
        projects::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_project: NewProject) -> QueryResult<Project> {
        diesel::insert_into(projects::table)
            .values(new_project)
            .get_result(connection)
    }

    pub fn update(
        connection: &PgConnection,
        mut new_project: NewProject,
        id: i32,
    ) -> QueryResult<Project> {
        let old_project: Project = Self::get(connection, id)?;
        if new_project.image == "".to_string() {
            // keep old image name in case of update without image
            new_project.image = old_project.image.clone();
        } else {
            delete_file(&old_project.image);
        }

        diesel::update(&old_project)
            .set(new_project)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Project> {
        // remove image
        let project: Project = Self::get(connection, id)?;
        delete_file(&project.image);

        diesel::delete(&project).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewProject {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::file("image"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("name"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("needed"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("collected"));
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

        let mut name = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("name") {
            name = &text.text;
        }

        let mut needed = 0;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("needed") {
            let amount = &text.text;
            needed = amount.parse().unwrap();
        }

        let mut collected = 0;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("collected") {
            let amount = &text.text;
            collected = amount.parse().unwrap();
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

        Success(NewProject {
            name: name.to_string(),
            image,
            needed,
            collected,
            description: description.to_string(),
            published,
        })
    }
}
