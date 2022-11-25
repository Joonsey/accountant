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
    use crate::Trade;
    use crate::schema::trade::id;
    use crate::schema::trade::dsl::trade;
    use crate::schema::trade::owner;
    use crate::*;
    use diesel::insert_into;

    pub fn trade_add(new_trade: rocket::serde::json::Json<Trade>)
    {
        let new_trade = NewTrade {
            owner      : &new_trade.owner,
            itemstring : &new_trade.item_string,
            itemname   : &new_trade.item_name,
            stacksize  : &new_trade.stack_size,
            quantity   : &new_trade.quantity,
            price      : &new_trade.price,
            otherplayer: &new_trade.other_player,
            player     : &new_trade.player,
            time       : &new_trade.time,
            source     : &new_trade.source
        };
        let conn = &mut establish_connection();
        insert_into(trade)
        .values(&new_trade)
        .execute(conn)
        .expect("Error saving new post");
    }

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
pub mod user_adapter
{
    use diesel::prelude::*;
    use crate::models::Users;
    use crate::schema::users::id;
    use crate::schema::users::name;
    use crate::schema::users::dsl::*;

    pub fn user_get_by_id(value : i32) -> rocket::serde::json::Json<Users>
    {
        let connection = &mut crate::establish_connection();
        let results: Users = users
            .filter(id.eq(value))
            .first::<Users>(connection)
            .expect("error loading trade");
        rocket::serde::json::Json(results)
    }

    pub fn user_get_by_name(value : String) -> rocket::serde::json::Json<Users>
    {
        let connection = &mut crate::establish_connection();
        let results: Users = users
            .filter(name.eq(value))
            .first::<Users>(connection)
            .expect("error loading trade");
        rocket::serde::json::Json(results)
    }
}
