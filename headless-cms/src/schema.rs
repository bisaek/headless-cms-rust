// @generated automatically by Diesel CLI.

diesel::table! {
    field_type (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        name -> Varchar,
        page_type -> Int4,
    }
}

diesel::table! {
    page (url) {
        url -> Varchar,
    }
}

diesel::table! {
    page_type (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(field_type -> page_type (page_type));

diesel::allow_tables_to_appear_in_same_query!(
    field_type,
    page,
    page_type,
);
