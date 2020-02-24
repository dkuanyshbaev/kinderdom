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

pub mod articles {
    use crate::models::article::{Article, NewArticle};
    handle!(Article, NewArticle, "admin/articles");

    // for test's sake -------------------------
    // use rocket::request::LenientForm;
    use rocket::response::Redirect;

    // ----------------------------------------------------------------
    use rocket::http::ContentType;
    use rocket::Data;
    use rocket_multipart_form_data::mime;
    use rocket_multipart_form_data::{
        FileField, MultipartFormData, MultipartFormDataError, MultipartFormDataField,
        MultipartFormDataOptions, RawField, TextField,
    };
    use rocket_raw_response::RawResponse;

    // #[post("/upload", data = "<data>")]
    // fn upload(content_type: &ContentType, data: Data) -> Result<RawResponse, &'static str> {
    // #[post("/test", data = "<new_article>")]
    #[post("/test", data = "<data>")]
    // pub fn test(new_article: LenientForm<NewArticle>) -> Redirect {
    pub fn test(content_type: &ContentType, data: Data) -> Redirect {
        // println!("-------------------------------------------");
        // println!("{:?}", new_article.image);
        // println!("{:?}", new_article.content);
        // println!("-------------------------------------------");

        // ----------------------------------------------------------------
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::text("title"));
        options.allowed_fields.push(
            MultipartFormDataField::file("image")
                // .size_limit(32 * 1024 * 1024)
                .content_type_by_string(Some(mime::IMAGE_STAR))
                .unwrap(),
        );
        options
            .allowed_fields
            .push(MultipartFormDataField::text("content"));
        options
            .allowed_fields
            .push(MultipartFormDataField::text("published"));

        // TODO: unwrap!!!
        let mut multipart_form_data =
            MultipartFormData::parse(content_type, data, options).unwrap();

        let title = multipart_form_data.texts.get("title");
        let image = multipart_form_data.files.get("image");
        let content = multipart_form_data.texts.get("content");
        let published = multipart_form_data.texts.get("published");

        if let Some(title) = title {
            match title {
                TextField::Single(text) => {
                    let _content_type = &text.content_type;
                    let _file_name = &text.file_name;
                    let _text = &text.text;
                    // You can now deal with the raw data.
                    println!("-------- title: {:?}", title)
                }
                TextField::Multiple(_texts) => {
                    // Because we only put one "text" field to the allowed_fields, this arm will not be matched.
                }
            }
        }

        if let Some(image) = image {
            match image {
                FileField::Single(file) => {
                    let _content_type = &file.content_type;
                    let file_name = &file.file_name;
                    let path = &file.path;
                    // You can now deal with the uploaded file.
                    // The file will be deleted automatically when the MultipartFormData instance is dropped.
                    // If you want to handle that file by your own, instead of killing it,
                    // just remove it out from the MultipartFormData instance.
                    // let image = multipart_form_data.files.remove("image");
                    println!("-------- we are here - can save image. {:?}", image);

                    // std::fs::copy("foo.txt", "bar.txt")?;
                    if let Some(file_path) = file_name {
                        match std::fs::copy(path, format!("static/upload/{}", file_path)) {
                            Ok(_) => (),
                            Err(e) => println!("{:?}", e),
                        }
                    }
                }
                FileField::Multiple(_files) => {
                    // Because we only put one "photo" field to the allowed_fields, this arm will not be matched.
                }
            }
        }

        if let Some(content) = content {
            match content {
                TextField::Single(text) => {
                    let _content_type = &text.content_type;
                    let _file_name = &text.file_name;
                    let _text = &text.text;
                    // You can now deal with the raw data.
                    println!("-------- content: {:?}", content)
                }
                TextField::Multiple(_texts) => {
                    // Because we only put one "text" field to the allowed_fields, this arm will not be matched.
                }
            }
        }

        if let Some(published) = published {
            match published {
                TextField::Single(text) => {
                    let _content_type = &text.content_type;
                    let _file_name = &text.file_name;
                    let _text = &text.text;
                    // You can now deal with the raw data.
                    println!("-------- published: {:?}", published)
                }
                TextField::Multiple(_bytes) => {
                    // Because we only put one "fingerprint" field to the allowed_fields, this arm will not be matched.
                }
            }
        }

        // ----------------------------------------------------------------
        // let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        //     Ok(multipart_form_data) => multipart_form_data,
        //     Err(err) => match err {
        //         // MultipartFormDataError::DataTooLargeError(_) => {
        //         //     return Err("The file is too large.");
        //         // }
        //         // MultipartFormDataError::DataTypeError(_) => {
        //         //     return Err("The file is not an image.");
        //         // }
        //         _ => panic!("{:?}", err),
        //     },
        // };

        // let image = multipart_form_data.raw.remove("image");

        // match image {
        //     Some(image) => match image {
        //         RawField::Single(raw) => {
        //             let content_type = raw.content_type;
        //             let file_name = raw.file_name.unwrap_or("Image".to_string());
        //             let data = raw.raw;
        //
        //             // Ok(RawResponse::from_vec(data, Some(file_name), content_type));
        //             println!("_____________________ we are here");
        //         }
        //         RawField::Multiple(_) => unreachable!(),
        //     },
        //     // None => Err("Please input a file."),
        //     _ => panic!("Errrorrrr!!"),
        // }
        // ----------------------------------------------------------------

        Redirect::to("/admin/articles/add")
    }

    use crate::models::article::MultipartError;
    type Result<T> = std::result::Result<T, MultipartError>;

    #[post("/upload", data = "<multipart>")]
    pub fn upload(multipart: Result<NewArticle>) -> String {
        println!("---------------------------------------------- we are here!!!!!");
        match multipart {
            Ok(m) => format!(
                "title: {}, content: {}, published: {}",
                m.title, m.content, m.published
            ),
            Err(e) => format!("Error: {}", e.reason),
        }
    }
}

pub mod profiles {
    use crate::models::profile::{NewProfile, Profile};
    handle!(Profile, NewProfile, "admin/profiles");
}

pub mod projects {
    use crate::models::project::{NewProject, Project};
    handle!(Project, NewProject, "admin/projects");
}

pub mod events {
    use crate::models::event::{Event, NewEvent};
    handle!(Event, NewEvent, "admin/events");
}
