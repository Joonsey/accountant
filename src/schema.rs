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
