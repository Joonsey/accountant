#[macro_use] extern crate rocket;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use accountant::*;

/*
pub mod trade_adapter
{

    pub fn trade_get_by_id(value : i32) -> rocket::serde::json::Json<Trade>
    {
        use accountant::schema::trade::dsl::*;
        let connection = &mut establish_connection();
        let results: Trade = trade
            .filter(id.eq(value))
            .first::<Trade>(connection)
            .expect("error loading trade");
        rocket::serde::json::Json(results)
    }

    pub fn trades_get_by_owner(value : String) -> rocket::serde::json::Json<Vec<Trade>>
    {
        use accountant::schema::trade::dsl::*;
        let connection = &mut establish_connection();
        let results: Vec<Trade> = trade
            .filter(owner.eq(value))
            .load::<Trade>(connection)
            .expect("error loading trade");
        rocket::serde::json::Json(results)
    }
}
*/
