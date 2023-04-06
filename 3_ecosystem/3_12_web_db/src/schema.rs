// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        labels -> Text,
    }
}
