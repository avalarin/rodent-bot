table! {
    users (id) {
        id -> Int4,
        tg_id -> Int8,
        tg_username -> Nullable<Varchar>,
        tg_fullname -> Nullable<Varchar>,
        active -> Bool,
    }
}
