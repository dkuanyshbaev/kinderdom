table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        image -> Varchar,
        content -> Text,
        welfare -> Bool,
        published -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    docs (id) {
        id -> Int4,
        file_name -> Varchar,
        description -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    donors (id) {
        id -> Int4,
        name -> Varchar,
        photo -> Varchar,
        description -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        image -> Varchar,
        needed -> Int4,
        collected -> Int4,
        description -> Text,
        is_vital -> Bool,
        published -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    orgs (id) {
        id -> Int4,
        name -> Varchar,
        logo -> Varchar,
        description -> Text,
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
        needed -> Int4,
        collected -> Int4,
        description -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        image -> Varchar,
        needed -> Int4,
        collected -> Int4,
        description -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    docs,
    donors,
    events,
    orgs,
    profiles,
    projects,
);
