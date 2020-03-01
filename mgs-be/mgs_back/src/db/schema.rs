table! {
    asset_snapshots (id) {
        id -> Integer,
        asset_id -> Integer,
        quantity -> Integer,
        date -> Date,
        created_at -> Timestamp,
    }
}

table! {
    assets (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        enabled -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    asset_snapshots,
    assets,
);
