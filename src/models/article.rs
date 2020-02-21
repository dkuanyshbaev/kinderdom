use super::schema::articles;
use chrono::NaiveDateTime;
use diesel::prelude::*;

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
