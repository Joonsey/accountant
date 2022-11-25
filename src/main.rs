#[macro_use] extern crate rocket;
use diesel::prelude::*;
use accountant::*;
use crate::trade_adapter::*;
use self::models::Trade;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get/<rid>")]
fn trades_get_specific(rid: i32) -> rocket::serde::json::Json<Trade>
{
    trade_get_by_id(rid)
}

#[get("/get")]
fn trades_get() -> rocket::serde::json::Json<Vec<Trade>>
{
    let connection = &mut establish_connection();
    use accountant::schema::trade::dsl::*;
    let results: Vec<Trade> = trade
        .limit(6)
        .load::<Trade>(connection)
        .expect("error loading trades");
    rocket::serde::json::Json(results)
}

#[post("/make", format="json", data = "<trade>")]
fn trades_make(trade: rocket::serde::json::Json<Trade>) -> String {

    let connection = &mut establish_connection();
    let result: Trade = create_trade(
        connection,
        &trade.owner,
        &trade.item_string ,
        &trade.item_name,
        &trade.stack_size,
        &trade.quantity,
        &trade.price,
        &trade.other_player,
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
    .mount("/trades", routes![trades_get_specific])
}
