table! {
    category_t (kind) {
        kind -> Text,
    }
}

table! {
    expense_t (id) {
        id -> Integer,
        cost -> Float,
        description -> Text,
        category -> Text,
        tags -> Text,
        ts -> Timestamp,
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
    expense_t,
    friend_t,
);
