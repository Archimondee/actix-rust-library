// @generated automatically by Diesel CLI.

diesel::table! {
    auths (id) {
        id -> Text,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        auth_id -> Text,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    auths,
    users,
);
