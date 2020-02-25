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
            new_item: crate::KinderResult<$nt>,
        ) -> crate::KinderResult<rocket::response::Redirect> {
            match new_item {
                Ok(item) => {
                    let _item = <$t>::insert(&connection, item)?;
                    Ok(rocket::response::Redirect::to(format!("/{}", $tp)))
                }
                Err(error) => {
                    println!("Error: {}", error);
                    Ok(rocket::response::Redirect::to(format!("/{}/add", $tp)))
                }
            }
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
            // new_item: rocket::request::Form<$nt>,
            new_item: crate::KinderResult<$nt>,
            id: i32,
        ) -> crate::KinderResult<rocket::response::Redirect> {
            match new_item {
                Ok(item) => {
                    let _item = <$t>::update(&connection, item, id)?;
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            }
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

pub mod articles {
    use crate::models::article::{Article, NewArticle};
    handle!(Article, NewArticle, "admin/articles");
}

// pub mod profiles {
//     use crate::models::profile::{NewProfile, Profile};
//     handle!(Profile, NewProfile, "admin/profiles");
// }
//
// pub mod projects {
//     use crate::models::project::{NewProject, Project};
//     handle!(Project, NewProject, "admin/projects");
// }
//
// pub mod events {
//     use crate::models::event::{Event, NewEvent};
//     handle!(Event, NewEvent, "admin/events");
// }
