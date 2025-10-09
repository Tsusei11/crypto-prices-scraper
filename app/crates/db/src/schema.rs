// @generated automatically by Diesel CLI.

diesel::table! {
    bars_1min (id) {
        id -> Int4,
        exchange -> Varchar,
        market -> Varchar,
        timestamp -> Timestamp,
        open -> Numeric,
        close -> Numeric,
        min -> Numeric,
        max -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(bars_1min,);
