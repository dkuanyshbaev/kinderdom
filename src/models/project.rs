use super::schema::projects;
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
#[table_name = "projects"]
pub struct NewProject {
    pub name: String,
    pub image: String,
    pub needed: i32,
    pub collected: i32,
    pub description: String,
    pub published: bool,
}

#[derive(Serialize, Queryable, Debug)]
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
        let old_project: Project = projects::table.find(id).get_result(connection)?;
        if new_project.image == "".to_string() {
            // we need to keep old image name in case of update without image
            new_project.image = old_project.image;
        } else {
            // remove old image in case of update with image
            if let Err(error) = std::fs::remove_file(format!("static/upload/{}", old_project.image))
            {
                println!("File error: {}", error);
            }
        }

        diesel::update(projects::table.find(id))
            .set(new_project)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Project> {
        // remove image
        let project: Project = projects::table.find(id).get_result(connection)?;
        if let Err(error) = std::fs::remove_file(format!("static/upload/{}", project.image)) {
            println!("File error: {}", error);
        }

        diesel::delete(projects::table.find(id)).get_result(connection)
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

        Success(NewProject {
            name: new_name.to_string(),
            image: new_image,
            description: new_description.to_string(),
            published: new_published_value,
        })
    }
}
