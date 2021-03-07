table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Varchar,
        author -> Int4,
        tags -> Varchar,
        permission -> Int4,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        pass -> Varchar,
        email -> Varchar,
        nickname -> Varchar,
        permission -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(posts, users,);
