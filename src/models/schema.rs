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

// table! {
//     profiles (id) {
//         id -> Int4,
//         name -> Varchar,
//         photo -> Varchar,
//         video -> Varchar,
//         description -> Text,
//         published -> Bool,
//         // created_at -> Timestamp,
//     }
// }
