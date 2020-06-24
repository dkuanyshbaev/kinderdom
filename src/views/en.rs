use crate::models::cause::Cause;
use crate::models::event::Event;
use crate::{Db, KinderResult};
use rocket_contrib::templates::Template;

#[derive(Serialize)]
pub struct IndexContext {
    causes: Vec<Cause>,
    events: Vec<Event>,
    stories: Vec<Event>,
}

#[get("/")]
pub fn index_en(connection: Db) -> KinderResult<Template> {
    let vitals = Cause::vital(&connection)?;
    let (last, stories) = Event::last(&connection)?;

    Ok(Template::render(
        "en/index_en",
        IndexContext {
            causes: vitals,
            events: last,
            stories: stories,
        },
    ))
}
