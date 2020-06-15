use crate::models::cause::Cause;
use crate::models::event::Event;

pub mod admin;
pub mod pages;

#[derive(Serialize)]
pub struct NoContext {}

#[derive(Serialize)]
pub struct ListContext<T> {
    items: Vec<T>,
}

#[derive(Serialize)]
pub struct PageContext<T, U, P> {
    total: U,
    page: P,
    items: Vec<T>,
}

#[derive(Serialize)]
pub struct ComplexContext<T, U> {
    item: U,
    items: Vec<T>,
}

#[derive(Serialize)]
pub struct IndexContext {
    causes: Vec<Cause>,
    events: Vec<Event>,
    stories: Vec<Event>,
}
