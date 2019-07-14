table! {
    keys (id) {
        id -> Int4,
        key -> Uuid,
        updated_on -> Timestamptz,
    }
}

table! {
    players (id, key_id) {
        id -> Int4,
        key_id -> Int4,
        name -> Varchar,
        score -> Int2,
        updated_on -> Timestamptz,
    }
}

joinable!(players -> keys (key_id));

allow_tables_to_appear_in_same_query!(
    keys,
    players,
);
