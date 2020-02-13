table! {
    profiles (id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        photo -> Varchar,
        video -> Varchar,
        description -> Text,
        published -> Bool,
        // created_at -> Timestamp,
    }
}
