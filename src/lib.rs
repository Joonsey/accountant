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
    owner : &str,
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
        owner,
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

// the boring part is over

pub mod trade_adapter
{
    use diesel::prelude::*;
    use crate::Trade;
    use crate::schema::trade::id;
    use crate::schema::trade::dsl::trade;
    use crate::schema::trade::owner;

    pub fn trade_get_by_id(value : i32) -> rocket::serde::json::Json<Trade>
    {
        let connection = &mut crate::establish_connection();
        let results: Trade = trade
            .filter(id.eq(value))
            .first::<Trade>(connection)
            .expect("error loading trade");
        rocket::serde::json::Json(results)
    }

    pub fn trades_get_by_owner(value : String) -> rocket::serde::json::Json<Vec<Trade>>
    {
        let connection = &mut crate::establish_connection();
        let results: Vec<Trade> = trade
            .filter(owner.eq(value))
            .load::<Trade>(connection)
            .expect("error loading trade");
        rocket::serde::json::Json(results)
    }
}
