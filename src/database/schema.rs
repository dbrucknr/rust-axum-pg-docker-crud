// @generated automatically by Diesel CLI.

diesel::table! {
    identities (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
