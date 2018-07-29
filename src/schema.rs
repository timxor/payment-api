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
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        user_name -> Varchar,
        email -> Varchar,
        public_key -> Varchar,
        private_key -> Varchar,
        eth_address -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    transfers,
    users,
);
