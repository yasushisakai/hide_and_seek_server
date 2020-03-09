table! {
    fence (id) {
        id -> Uuid,
        ts -> Array<Timestamp>,
        lat -> Array<Float4>,
        lng -> Array<Float4>,
        tcount -> Int4,
        fcount -> Int4,
    }
}

table! {
    proof (id) {
        id -> Text,
        fid -> Uuid,
        result -> Bool,
    }
}

joinable!(proof -> fence (fid));

allow_tables_to_appear_in_same_query!(
    fence,
    proof,
);
