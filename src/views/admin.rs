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
        #[get("/<id>")]
        pub fn edit(
            connection: crate::Db,
            id: i32,
        ) -> crate::KinderResult<rocket_contrib::templates::Template> {
            let item = <$t>::get(&connection, id)?;

            Ok(rocket_contrib::templates::Template::render(
                format!("{}/edit", $tp),
                item,
            ))
        }

        // update item
        #[put("/<id>", data = "<new_item>")]
        pub fn update(
            connection: crate::Db,
            new_item: rocket::request::Form<$nt>,
            id: i32,
        ) -> crate::KinderResult<rocket::response::Redirect> {
            let _item = <$t>::update(&connection, new_item.into_inner(), id)?;

            Ok(rocket::response::Redirect::to(format!("/{}/{}", $tp, id)))
        }

        // delete item
        #[delete("/<id>")]
        pub fn delete(
            connection: crate::Db,
            id: i32,
        ) -> crate::KinderResult<rocket::response::Redirect> {
            let _item = <$t>::delete(&connection, id)?;

            Ok(rocket::response::Redirect::to(format!("/{}", $tp)))
        }
    };
}

pub mod profiles {
    trace_macros!(true);
    use crate::models::profile::{NewProfile, Profile};
    handle!(Profile, NewProfile, "admin/profiles");
    trace_macros!(false);
}
