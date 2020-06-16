#[derive(Serialize)]
struct Select {
    id: i32,
    name: String,
}

macro_rules! handle {
    ($t:ty, $nt:ty, $tp:expr) => {
        // list of items
        #[get("/")]
        pub fn list(
            _admin: crate::auth::Admin,
            connection: crate::Db,
        ) -> crate::KinderResult<rocket_contrib::templates::Template> {
            let items = <$t>::all(&connection)?;
            let context: crate::views::ListContext<$t> = crate::views::ListContext { items };

            Ok(rocket_contrib::templates::Template::render(
                format!("{}/list", $tp),
                context,
            ))
        }

        // show add form
        #[get("/add")]
        pub fn add(
            _admin: crate::auth::Admin,
            connection: crate::Db,
        ) -> crate::KinderResult<rocket_contrib::templates::Template> {
            Ok(rocket_contrib::templates::Template::render(
                format!("{}/add", $tp),
                crate::views::ListContext {
                    items: crate::models::cat::Cat::all(&connection)?,
                },
            ))
        }

        // create item
        #[post("/", data = "<new_item>")]
        pub fn create(
            _admin: crate::auth::Admin,
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
            _admin: crate::auth::Admin,
            connection: crate::Db,
            id: i32,
        ) -> crate::KinderResult<rocket_contrib::templates::Template> {
            // let item = <$t>::get(&connection, id)?;
            // Ok(rocket_contrib::templates::Template::render(
            //     format!("{}/edit", $tp),
            //     item,
            // ))

            // items here it's categories for edit's form select
            Ok(rocket_contrib::templates::Template::render(
                format!("{}/edit", $tp),
                crate::views::ComplexContext {
                    item: <$t>::get(&connection, id)?,
                    items: crate::models::cat::Cat::all(&connection)?,
                },
            ))
        }

        // update item
        // post here instead of put - because of multipart
        #[post("/<id>", data = "<new_item>")]
        pub fn update(
            _admin: crate::auth::Admin,
            connection: crate::Db,
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
            _admin: crate::auth::Admin,
            connection: crate::Db,
            id: i32,
        ) -> crate::KinderResult<rocket::response::Redirect> {
            let _item = <$t>::delete(&connection, id)?;

            Ok(rocket::response::Redirect::to(format!("/{}", $tp)))
        }
    };
}

pub mod profiles {
    use crate::models::profile::{NewProfile, Profile};
    handle!(Profile, NewProfile, "admin/profiles");
}

pub mod events {
    use crate::models::event::{Event, NewEvent};
    handle!(Event, NewEvent, "admin/events");
}

pub mod cats {
    use crate::models::cat::{Cat, NewCat};
    handle!(Cat, NewCat, "admin/cats");
}

pub mod causes {
    use crate::models::cause::{Cause, NewCause};
    handle!(Cause, NewCause, "admin/causes");
}

pub mod reports {
    use crate::models::report::{NewReport, Report};
    handle!(Report, NewReport, "admin/reports");
}
