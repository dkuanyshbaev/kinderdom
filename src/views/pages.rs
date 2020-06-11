use crate::auth::{Admin, LoginForm};
use crate::models::cause::Cause;
use crate::models::donor::Donor;
use crate::models::event::Event;
use crate::models::payment::{
    Amount, PaymentBody, PaymentForm, PaymentResponse, RequestConfirmation,
};
use crate::models::profile::Profile;
use crate::models::report::Report;
use crate::views::{IndexContext, ListContext, NoContext};
use crate::{Config, Db, KinderResult};
use base64::encode;
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use uuid::Uuid;

#[get("/")]
pub fn index(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/index",
        IndexContext {
            causes: Cause::vital(&connection)?,
            events: Event::last(&connection)?,
            stories: Event::last(&connection)?,
            donors: Donor::last(&connection)?,
        },
    ))
}

#[get("/events")]
pub fn events(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/events",
        ListContext {
            items: Event::published(&connection)?,
        },
    ))
}

#[get("/events/<id>")]
pub fn event_details(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/event_details",
        Event::get(&connection, id)?,
    ))
}

#[get("/causes")]
pub fn causes(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/causes",
        ListContext {
            items: Cause::published(&connection)?,
        },
    ))
}

#[get("/causes/<id>")]
pub fn cause_details(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/cause_details",
        Cause::get(&connection, id)?,
    ))
}

#[get("/profiles")]
pub fn profiles(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/profiles",
        ListContext {
            items: Profile::published(&connection)?,
        },
    ))
}

#[get("/profiles/<id>")]
pub fn profile_details(connection: Db, id: i32) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/profile_details",
        Profile::get(&connection, id)?,
    ))
}

#[get("/reports")]
pub fn reports(connection: Db) -> KinderResult<Template> {
    Ok(Template::render(
        "pages/reports",
        ListContext {
            items: Report::all(&connection)?,
        },
    ))
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("pages/about", NoContext {})
}

#[get("/admin")]
pub fn admin(_admin: Admin) -> Redirect {
    Redirect::to("/admin/events")
}

#[get("/login")]
pub fn login_page() -> Template {
    Template::render("pages/login", NoContext {})
}

#[post("/login", data = "<login_form>")]
pub fn login(
    mut cookies: Cookies,
    config: State<Config>,
    login_form: Form<LoginForm>,
) -> KinderResult<Redirect> {
    if login_form.password == config.secret {
        cookies.add_private(Cookie::new("admin", 1.to_string()));

        Ok(Redirect::to("/admin"))
    } else {
        Ok(Redirect::to("/login"))
    }
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("admin"));

    Redirect::to("/login")
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

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("pages/404", NoContext {})
}

#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to("/login")
}

#[catch(422)]
pub fn unprocessable() -> Redirect {
    Redirect::to("/")
}
