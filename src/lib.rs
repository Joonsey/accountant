pub mod models;
pub mod schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::models::{NewTrade, Trade};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_trade(
    conn: &mut PgConnection,
    itemstring : &str,
    itemname  : &str,
    stacksize : &i32,
    quantity : &i32,
    price: &i32,
    otherplayer: &str ,
    player: &str,
    time: &i32,
    source: &str) -> Trade {
    use crate::schema::trade;

    let new_trade = NewTrade {
        itemstring,
        itemname,
        stacksize,
        quantity,
        price,
        otherplayer,
        player,
        time,
        source
    };

    diesel::insert_into(trade::table)
        .values(&new_trade)
        .get_result(conn)
        .expect("Error saving new post")
}
