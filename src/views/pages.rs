use crate::models::cat::Cat;
use crate::models::cause::Cause;
use crate::models::event::Event;
use crate::models::payment::{
    Amount, PaymentBody, PaymentForm, PaymentResponse, RequestConfirmation,
};
use crate::models::profile::Profile;
use crate::models::report::Report;
use crate::models::search::SearchForm;
use crate::{Config, Db, KinderResult};
use base64::encode;
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use uuid::Uuid;

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
pub fn index(connection: Db) -> KinderResult<Template> {
    let causes = Cause::vital(&connection, false)?;
    let (events, stories) = Event::last(&connection, false)?;

    Ok(Template::render(
        "pages/index",
        IndexContext {
            causes,
            events,
            stories,
        },
    ))
}

#[get("/events?<page>&<cat>")]
pub fn events(connection: Db, page: Option<u8>, cat: Option<u8>) -> KinderResult<Template> {
    let (total, page, cat, events) = Event::paginated_by_cat(&connection, page, cat, false)?;

    Ok(Template::render(
        "pages/events",
        EventsContext {
            total,
            page,
            cat,
            events,
        },
    ))
}

#[get("/events/<id>")]
pub fn event_details(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/event_details",
        EventContext {
            cats: Cat::ru(&connection)?,
            event: Event::get(&connection, id)?,
            causes: Cause::vital(&connection, false)?,
        },
    ))
}

#[get("/causes")]
pub fn causes(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/causes",
        CausesContext {
            causes: Cause::published(&connection, false)?,
        },
    ))
}

#[get("/causes/<id>")]
pub fn cause_details(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/cause_details",
        CauseContext {
            cause: Cause::get(&connection, id)?,
            vitals: Cause::vital(&connection, false)?,
        },
    ))
}

#[get("/profiles?<page>")]
pub fn profiles(connection: Db, page: Option<u8>) -> KinderResult<Template> {
    let (total, page, profiles) = Profile::paginated(&connection, page, false)?;

    Ok(Template::render(
        "pages/profiles",
        ProfilesContext {
            total,
            page,
            profiles,
        },
    ))
}

#[get("/profiles/<id>")]
pub fn profile_details(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/profile_details",
        ProfileContext {
            profile: Profile::get(&connection, id)?,
            vitals: Cause::vital(&connection, false)?,
        },
    ))
}

// #[get("/reports?<page>")]
// pub fn reports(connection: Db, page: Option<u8>) -> KinderResult<Template> {
#[get("/reports")]
pub fn reports(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/reports",
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
pub fn about() -> Template {
    Template::render("pages/about", NoContext {})
}

#[post("/search", data = "<search_form>")]
pub fn search(connection: Db, search_form: Form<SearchForm>) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/results",
        SearchContext {
            events: Event::search(&connection, search_form.term.to_owned())?,
        },
    ))
}

#[post("/payment", data = "<payment_form>")]
pub fn payment(config: State<Config>, payment_form: Form<PaymentForm>) -> KinderResult<Redirect> {
    let idempotence_key = Uuid::new_v4();
    let auth_credentials = encode(config.yandex_credentials.to_owned());
    let body = PaymentBody {
        amount: Amount {
            value: payment_form.amount.to_owned(),
            currency: "RUB".to_string(),
        },
        capture: true,
        confirmation: RequestConfirmation {
            r#type: "redirect".to_string(),
            return_url: "https://kinderdom.org/thankyou".to_string(),
        },
        description: payment_form.description.to_owned(),
    };

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://payment.yandex.net/api/v3/payments")
        .header("Idempotence-Key", idempotence_key.to_string())
        .header(AUTHORIZATION, format!("Basic {}", auth_credentials))
        .json(&body)
        .send();

    match res {
        Ok(r) => match r.status() {
            StatusCode::OK => {
                if let Ok(res_json) = r.json::<PaymentResponse>() {
                    Ok(Redirect::to(res_json.confirmation.confirmation_url))
                } else {
                    println!("Can't find confirmation url");
                    Ok(Redirect::to("/"))
                }
            }
            s => {
                println!("Response status: {:?}", s);
                Ok(Redirect::to("/"))
            }
        },
        Err(e) => {
            println!("Payment response: {}", e);
            Ok(Redirect::to("/"))
        }
    }
}

#[get("/thankyou")]
pub fn thankyou() -> Template {
    Template::render("pages/thankyou", NoContext {})
}

#[get("/paylink")]
pub fn paylink() -> Template {
    Template::render("pages/paylink", NoContext {})
}
