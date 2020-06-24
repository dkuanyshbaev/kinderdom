use super::schema::{cats, events};
use super::utils::{delete_file, save_file, uuid_file_name};
use crate::errors::KinderError;
use crate::models::cat::Cat;
use chrono::NaiveDateTime;
use diesel::{prelude::*, sql_query};
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
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
    pub en: bool,
    pub cat_id: i32,
}

#[derive(QueryableByName, Serialize, Queryable, Identifiable, Clone, Debug)]
#[table_name = "events"]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub lead: String,
    pub cover: String,
    pub content: String,
    pub published: bool,
    pub en: bool,
    pub cat_id: i32,
    pub created_at: NaiveDateTime,
}

impl Event {
    pub fn all(connection: &PgConnection) -> QueryResult<Vec<Event>> {
        events::table.order(events::id.desc()).load(connection)
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

    pub fn published(connection: &PgConnection) -> QueryResult<Vec<Event>> {
        events::table
            .filter(events::published.eq(true))
            .order(events::id.desc())
            .load(connection)
    }

    pub fn paginated_by_cat(
        connection: &PgConnection,
        page: Option<u8>,
        cat: Option<u8>,
    ) -> QueryResult<(u8, u8, u8, Vec<Event>)> {
        let mut cat_num = 0;
        if let Some(c) = cat {
            cat_num = c;
        }

        let mut page_num = 0;
        if let Some(p) = page {
            page_num = p;
        }

        match Self::published(connection) {
            Ok(mut events) => {
                if cat_num != 0 {
                    events.retain(|e| e.cat_id == cat_num as i32);
                }

                let max = events.len();
                let total = (max as i64 / EVENTS_PER_PAGE + 1) as u8;

                let mut offset = page_num as usize * EVENTS_PER_PAGE as usize;
                if offset > max {
                    offset = max;
                }

                let mut limit = offset + EVENTS_PER_PAGE as usize;
                if limit > max {
                    limit = max;
                }

                let events = events[offset..limit].to_vec();

                Ok((total, page_num, cat_num, events))
            }
            Err(error) => Err(error),
        }
    }

    pub fn last(connection: &PgConnection) -> QueryResult<(Vec<Event>, Vec<Event>)> {
        let cat = cats::table
            .filter(cats::name.eq("Истории успеха"))
            .first::<Cat>(connection)?;

        match Self::published(connection) {
            Ok(events) => {
                let mut last: Vec<Event> = vec![];
                let mut stories: Vec<Event> = vec![];

                for e in events {
                    if e.cat_id == cat.id {
                        stories.push(e);
                    } else {
                        last.push(e);
                    }
                    if last.len() > 3 && stories.len() > 3 {
                        break;
                    }
                }

                if last.len() > 3 {
                    last = last[0..3].to_vec();
                }

                if stories.len() > 3 {
                    stories = stories[0..3].to_vec();
                }

                Ok((last, stories))
            }
            Err(error) => Err(error),
        }
    }

    pub fn search(connection: &PgConnection, term: String) -> QueryResult<Vec<Event>> {
        sql_query(format!(
            "select * from events
            where to_tsvector(title) || to_tsvector(content)
            @@ plainto_tsquery('{}') order by id desc",
            sql_lexer::sanitize_string(term)
        ))
        .load(connection)
    }
}

// we need this custom impl for multipart form
impl FromDataSimple for NewEvent {
    type Error = KinderError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
            MultipartFormDataField::file("cover"),
            MultipartFormDataField::text("title"),
            MultipartFormDataField::text("cat"),
            MultipartFormDataField::text("lead"),
            MultipartFormDataField::text("content"),
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

        let mut cover = "".to_string();
        if let Some(file_fields) = multipart_form.files.get("cover") {
            let file_field = &file_fields[0];
            let file_name = &file_field.file_name;
            let path = &file_field.path;

            if let Some(file_path) = file_name {
                // check if it's update or create?
                if file_path != "" {
                    cover = uuid_file_name(file_path);
                    save_file(path, &cover);
                }
            }
        }

        let mut cat_id = 1;
        if let Some(mut text_fields) = multipart_form.texts.remove("cat") {
            let text_field = text_fields.remove(0);
            let amount = text_field.text;
            cat_id = amount.parse().unwrap();
        }

        let mut title = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("title") {
            let text_field = text_fields.remove(0);
            title = text_field.text;
        }

        let mut lead = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("lead") {
            let text_field = text_fields.remove(0);
            lead = text_field.text;
        }

        let mut content = "".to_string();
        if let Some(mut text_fields) = multipart_form.texts.remove("content") {
            let text_field = text_fields.remove(0);
            content = text_field.text;
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

        Success(NewEvent {
            cat_id,
            title,
            lead,
            cover,
            content,
            en,
            published,
        })
    }
}
