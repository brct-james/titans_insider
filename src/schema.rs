// @generated automatically by Diesel CLI.

diesel::table! {
    item_hist (uuid) {
        uuid -> Text,
        item_id -> Int4,
        t_type -> Text,
        uid -> Text,
        tag1 -> Nullable<Text>,
        tag2 -> Nullable<Text>,
        tag3 -> Nullable<Text>,
        gold_qty -> Int4,
        gems_qty -> Int4,
        created -> Nullable<Text>,
        tier -> Nullable<Int4>,
        item_order -> Nullable<Int4>,
        city_id -> Nullable<Int4>,
        gold_price -> Int4,
        gems_price -> Int4,
        request_cycle -> Int4,
        created_at -> Text,
        updated_at -> Text,
        db_timestamp -> Int8,
    }
}
