table! {
    causes (id) {
        id -> Int4,
        name -> Varchar,
        image -> Varchar,
        video -> Varchar,
        needed -> Int4,
        collected -> Int4,
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
        position -> Varchar,
        quote -> Text,
        created_at -> Timestamp,
    }
}

table! {
    events (id) {
        id -> Int4,
        title -> Varchar,
        lead -> Varchar,
        cover -> Varchar,
        content -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    reports (id) {
        id -> Int4,
        pdf -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    causes,
    donors,
    events,
    reports,
);
