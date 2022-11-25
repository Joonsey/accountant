#[macro_use] extern crate rocket;

use diesel::prelude::*;
use accountant::*;
use::serde::{Deserialize, Serialize};

use self::models::Trade;

#[derive(Deserialize, Serialize, Eq, Debug, PartialEq)]
struct JsonTrade {
    owner : String,
    itemstring : String,
    itemname  : String,
    stacksize : i32,
    quantity : i32,
    price: i32,
	otherplayer: String ,
	player: String,
	time: i32,
	source: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get")]
fn trades_get() -> rocket::serde::json::Json<Vec<Trade>>
{
    use accountant::schema::trade::dsl::*;
    let connection = &mut establish_connection();
    let results: Vec<Trade> = trade
        .limit(6)
        .load::<Trade>(connection)
        .expect("error loading trades");
    //let top_trade: Trade = results.get_mut(0);
    rocket::serde::json::Json(results)
}

#[post("/make", format="json", data = "<trade>")]
fn trades_make(trade: rocket::serde::json::Json<JsonTrade>) -> String {

    let connection = &mut establish_connection();
    let result: Trade = create_trade(
        connection,
        &trade.owner,
        &trade.itemstring ,
        &trade.itemname,
        &trade.stacksize,
        &trade.quantity,
        &trade.price,
        &trade.otherplayer,
        &trade.player,
        &trade.time,
        &trade.source);
    format!("{}", result.id)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/trades", routes![trades_make])
    .mount("/trades", routes![trades_get])
}
