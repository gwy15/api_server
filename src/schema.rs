table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
        is_disabled -> Bool,
        last_login -> Timestamptz,
        token_valid_after -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
