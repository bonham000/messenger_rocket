table! {
    messages (id) {
        id -> Int4,
        message -> Varchar,
        author -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    posts,
);
