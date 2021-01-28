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

allow_tables_to_appear_in_same_query!(users,);
