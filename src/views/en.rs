use crate::models::cat::Cat;
use crate::models::cause::Cause;
use crate::models::event::Event;
use crate::models::profile::Profile;
use crate::models::report::Report;
use crate::models::search::SearchForm;
use crate::{Db, KinderResult};
use rocket::request::Form;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
pub struct IndexContext {
    causes: Vec<Cause>,
    events: Vec<Event>,
    stories: Vec<Event>,
}

#[derive(Serialize)]
pub struct EventsContext {
    total: u8,
    page: u8,
    cat: u8,
    events: Vec<Event>,
}

#[derive(Serialize)]
pub struct EventContext {
    cats: Vec<Cat>,
    event: Event,
    causes: Vec<Cause>,
}

#[derive(Serialize)]
pub struct CausesContext {
    causes: Vec<Cause>,
}

#[derive(Serialize)]
pub struct CauseContext {
    cause: Cause,
    vitals: Vec<Cause>,
}

#[derive(Serialize)]
pub struct ProfilesContext {
    total: u8,
    page: u8,
    profiles: Vec<Profile>,
}

#[derive(Serialize)]
pub struct ProfileContext {
    profile: Profile,
    vitals: Vec<Cause>,
}

#[derive(Serialize)]
pub struct ReportsContext {
    // total: u8,
    // page: u8,
    reports: Vec<Report>,
}

#[derive(Serialize)]
pub struct NoContext {}

#[derive(Serialize)]
pub struct SearchContext {
    events: Vec<Event>,
}

#[get("/")]
pub fn index_en(connection: Db) -> KinderResult<Template> {
    let vitals = Cause::vital(&connection, true)?;
    let (last, stories) = Event::last(&connection, true)?;

    Ok(Template::render(
        "en/index_en",
        IndexContext {
            causes: vitals,
            events: last,
            stories: stories,
        },
    ))
}

#[get("/events?<page>&<cat>")]
pub fn events_en(connection: Db, page: Option<u8>, cat: Option<u8>) -> KinderResult<Template> {
    let (total, page, cat, events) = Event::paginated_by_cat(&connection, page, cat, true)?;

    Ok(Template::render(
        "en/events_en",
        EventsContext {
            total,
            page,
            cat,
            events,
        },
    ))
}

#[get("/events/<id>")]
pub fn event_details_en(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "en/event_details_en",
        EventContext {
            cats: Cat::en(&connection)?,
            event: Event::get(&connection, id)?,
            causes: Cause::vital(&connection, true)?,
        },
    ))
}

#[get("/causes")]
pub fn causes_en(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "en/causes_en",
        CausesContext {
            causes: Cause::published(&connection, true)?,
        },
    ))
}

#[get("/causes/<id>")]
pub fn cause_details_en(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "en/cause_details_en",
        CauseContext {
            cause: Cause::get(&connection, id)?,
            vitals: Cause::vital(&connection, true)?,
        },
    ))
}

#[get("/profiles?<page>")]
pub fn profiles_en(connection: Db, page: Option<u8>) -> KinderResult<Template> {
    let (total, page, profiles) = Profile::paginated(&connection, page, true)?;

    Ok(Template::render(
        "en/profiles_en",
        ProfilesContext {
            total,
            page,
            profiles,
        },
    ))
}

#[get("/profiles/<id>")]
pub fn profile_details_en(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "en/profile_details_en",
        ProfileContext {
            profile: Profile::get(&connection, id)?,
            vitals: Cause::vital(&connection, true)?,
        },
    ))
}

// #[get("/reports?<page>")]
// pub fn reports(connection: Db, page: Option<u8>) -> KinderResult<Template> {
#[get("/reports")]
pub fn reports_en(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "en/reports_en",
        ReportsContext {
            reports: Report::all(&connection)?,
        },
    ))

    // let (total, page, reports) = Report::paginated(&connection, page)?;
    //
    // Ok(Template::render(
    //     "pages/reports",
    //     ReportsContext {
    //         total,
    //         page,
    //         reports,
    //     },
    // ))
}

#[get("/about")]
pub fn about_en() -> Template {
    Template::render("en/about_en", NoContext {})
}

#[post("/search", data = "<search_form>")]
pub fn search_en(connection: Db, search_form: Form<SearchForm>) -> KinderResult<Template> {
    Ok(Template::render(
        "en/results_en",
        SearchContext {
            events: Event::search(&connection, search_form.term.to_owned())?,
        },
    ))
}
