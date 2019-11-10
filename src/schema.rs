table! {
    category_t (kind) {
        kind -> Text,
    }
}

table! {
    friend_t (name) {
        name -> Text,
        upi_id -> Text,
        phone -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    category_t,
    friend_t,
);
