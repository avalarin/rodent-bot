table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    stored_contexts (id) {
        id -> Int4,
        user_id -> Int4,
        updated_on -> Timestamp,
        data -> Jsonb,
    }
}

table! {
    user_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        tg_id -> Int8,
        tg_username -> Nullable<Varchar>,
        tg_fullname -> Nullable<Varchar>,
        active -> Bool,
        email -> Nullable<Varchar>,
    }
}

joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    roles,
    stored_contexts,
    user_roles,
    users,
);
