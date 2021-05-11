table! {
    categories (c_id) {
        c_id -> Int4,
        genre -> Varchar,
    }
}

table! {
    mangas (id) {
        id -> Int4,
        descriptions -> Varchar,
        reading_status -> Varchar,
        genre -> Varchar,
        completed -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password -> Text,
        created_on -> Timestamp,
        last_login -> Nullable<Timestamp>,
        timezone -> Text,
        email_verified -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    mangas,
    users,
);