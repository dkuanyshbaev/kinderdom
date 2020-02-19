#[derive(Serialize)]
pub struct TemplateContext<T> {
    items: Vec<T>,
}

macro_rules! handle {
    ($t:ty, $nt:ty, $tp:expr) => {
        // list of items
        #[get("/")]
        pub fn list(
            connection: crate::Db,
        ) -> crate::KinderResult<rocket_contrib::templates::Template> {
            let items = <$t>::all(&connection)?;
            let context: crate::views::admin::TemplateContext<$t> =
                crate::views::admin::TemplateContext { items };

            Ok(rocket_contrib::templates::Template::render(
                format!("{}/list", $tp),
                context,
            ))
        }

        // show add form
        #[get("/add")]
        pub fn add() -> rocket_contrib::templates::Template {
            rocket_contrib::templates::Template::render(
                format!("{}/add", $tp),
                crate::views::NoContext {},
            )
        }

        // create item
        #[post("/", data = "<new_item>")]
        pub fn create(
            connection: crate::Db,
            new_item: rocket::request::Form<$nt>,
        ) -> crate::KinderResult<rocket::response::Redirect> {
            let _item = <$t>::insert(&connection, new_item.into_inner())?;

            Ok(rocket::response::Redirect::to(format!("/{}", $tp)))
        }

        // show edit form
        // update item
        // delete item
    };
}

pub mod profiles {
    trace_macros!(true);
    handle!(
        crate::models::profile::Profile,
        crate::models::profile::NewProfile,
        "admin/profiles"
    );
    trace_macros!(false);
}
