table! {
    cats (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    causes (id) {
        id -> Int4,
        name -> Varchar,
        image -> Varchar,
        video -> Varchar,
        needed -> Int4,
        collected -> Int4,
        description -> Text,
        vital -> Bool,
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
        cat_id -> Int4,
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
    reports (id) {
        id -> Int4,
        url -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(events -> cats (cat_id));

allow_tables_to_appear_in_same_query!(
    cats,
    causes,
    donors,
    events,
    profiles,
    reports,
);
