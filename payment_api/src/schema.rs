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
