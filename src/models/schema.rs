table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        image -> Varchar,
        content -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    events (id) {
        id -> Int4,
        published -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    profiles (id) {
        id -> Int4,
        name -> Varchar,
        photo -> Varchar,
        video -> Varchar,
        description -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    projects (id) {
        id -> Int4,
        published -> Bool,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    events,
    profiles,
    projects,
);
