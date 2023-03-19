// @generated automatically by Diesel CLI.

diesel::table! {
    public_shares (id) {
        id -> Text,
        file_path -> Text,
        created -> Timestamp,
    }
}

diesel::table! {
    users (name) {
        name -> Text,
        password_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(public_shares, users,);
