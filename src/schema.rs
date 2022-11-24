// @generated automatically by Diesel CLI.

diesel::table! {
    trade (id) {
        id -> Int4,
        owner -> Nullable<Varchar>,
        itemstring -> Nullable<Varchar>,
        itemname -> Nullable<Varchar>,
        stacksize -> Nullable<Int4>,
        quantity -> Nullable<Int4>,
        price -> Nullable<Int4>,
        otherplayer -> Nullable<Varchar>,
        player -> Nullable<Varchar>,
        time -> Nullable<Int4>,
        source -> Nullable<Varchar>,
    }
}
