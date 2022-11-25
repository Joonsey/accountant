// @generated automatically by Diesel CLI.

diesel::table! {
    trade (id) {
        id -> Int4,
        owner -> Varchar,
        itemstring -> Varchar,
        itemname -> Varchar,
        stacksize -> Int4,
        quantity -> Int4,
        price -> Int4,
        otherplayer -> Varchar,
        player -> Varchar,
        time -> Int4,
        source -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    trade,
    users,
);
