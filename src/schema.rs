table! {
    persons (id) {
        id -> Int4,
        name -> Varchar,
        photo -> Varchar,
        video -> Varchar,
        story -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}
