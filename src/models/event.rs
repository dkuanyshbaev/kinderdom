use super::schema::events;
use super::utils::{delete_file, save_file, uuid_file_name};
use crate::errors::KinderError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    FileField, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, TextField,
};

const EVENTS_PER_PAGE: i64 = 6;

#[derive(Serialize, Insertable, FromForm, AsChangeset)]
#[table_name = "events"]
pub struct NewEvent {
    pub title: String,
    pub lead: String,
    pub cover: String,
    pub content: String,
    pub published: bool,
    pub cat_id: i32,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub lead: String,
    pub cover: String,
    pub content: String,
    pub published: bool,
    pub cat_id: i32,
    pub created_at: NaiveDateTime,
}

impl Event {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Event>> {
        events::table.order(events::id.desc()).load(connection)
    }

    pub fn published(connection: &PgConnection, page: i64, cat: i32) -> QueryResult<Vec<Event>> {
        if cat > 0 {
            events::table
                .filter(events::published.eq(true))
                .filter(events::cat_id.eq(cat))
                .offset(page * EVENTS_PER_PAGE)
                .limit(EVENTS_PER_PAGE)
                .order(events::id.desc())
                .load(connection)
        } else {
            events::table
                .filter(events::published.eq(true))
                .offset(page * EVENTS_PER_PAGE)
                .limit(EVENTS_PER_PAGE)
                .order(events::id.desc())
                .load(connection)
        }
    }

    pub fn pages_total(connection: &PgConnection, cat: i32) -> usize {
        let mut total = 0;
        if cat > 0 {
            if let Ok(es) = events::table
                .filter(events::cat_id.eq(cat))
                .filter(events::published.eq(true))
                .load::<Event>(connection)
            {
                total = es.len();
            }
        } else {
            if let Ok(es) = events::table
                .filter(events::published.eq(true))
                .load::<Event>(connection)
            {
                total = es.len();
            }
        }

        total / EVENTS_PER_PAGE as usize + 1
    }

    pub fn last(connection: &PgConnection) -> QueryResult<Vec<Event>> {
        events::table
            .filter(events::published.eq(true))
            .limit(3)
            .order(events::id.desc())
            .load(connection)
    }

    pub fn stories(connection: &PgConnection) -> QueryResult<Vec<Event>> {
        events::table
            .filter(events::published.eq(true))
            .limit(3)
            .order(events::id.desc())
            .load(connection)
    }

    pub fn get(connection: &PgConnection, id: i32) -> QueryResult<Event> {
        events::table.find(id).get_result(connection)
    }

    pub fn insert(connection: &PgConnection, new_event: NewEvent) -> QueryResult<Event> {
        diesel::insert_into(events::table)
            .values(new_event)
            .get_result(connection)
    }

    pub fn update(
        connection: &PgConnection,
        mut new_event: NewEvent,
        id: i32,
    ) -> QueryResult<Event> {
        let old_event: Event = Self::get(connection, id)?;
        if new_event.cover == "".to_string() {
            // keep old cover name in case of update without cover
            new_event.cover = old_event.cover.clone();
        } else {
            delete_file(&old_event.cover);
        }

        diesel::update(&old_event)
            .set(new_event)
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, id: i32) -> QueryResult<Event> {
        // remove cover
        let event: Event = Self::get(connection, id)?;
        delete_file(&event.cover);

        diesel::delete(&event).get_result(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewEvent {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::file("cover"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("title"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("cat"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("lead"));
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

        let mut cover = "".to_string();
        if let Some(FileField::Single(file)) = multipart_form.files.get("cover") {
            let file_name = &file.file_name;
            let path = &file.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    cover = uuid_file_name(file_path);
                    save_file(path, &cover);
                }
            }
        }

        let mut cat = 1;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("cat") {
            let amount = &text.text;
            cat = amount.parse().unwrap();
        }

        let mut title = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("title") {
            title = &text.text;
        }

        let mut lead = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("lead") {
            lead = &text.text;
        }

        let mut content = "";
        if let Some(TextField::Single(text)) = multipart_form.texts.get("content") {
            content = &text.text;
        }

        let mut published = false;
        if let Some(TextField::Single(text)) = multipart_form.texts.get("published") {
            if &text.text == "on" {
                published = true;
            }
        }

        Success(NewEvent {
            cat_id: cat,
            title: title.to_string(),
            lead: lead.to_string(),
            cover,
            content: content.to_string(),
            published,
        })
    }
}
