table! {
    transfers (id) {
        id -> Int4,
        amount -> Text,
        currency -> Text,
        to_name -> Text,
        to_number -> Text,
        to_email -> Text,
        complete -> Bool,
    }
}

table! {
    users (id) {
        id -> Nullable<Int4>,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    transfers,
    users,
);
