use diesel::prelude::*;
use crate::schema::trade;
use crate::schema::users;
use::serde::{Deserialize, Serialize};

#[derive(Deserialize, Queryable, Serialize)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub password: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a String,
    pub password: &'a String
}

#[derive(Deserialize, Queryable, Serialize)]
pub struct Trade {
    pub id: i32,
    pub owner: String,
    pub item_string : String,
    pub item_name  : String,
    pub stack_size : i32,
    pub quantity : i32,
    pub price: i32,
	pub other_player: String,
	pub player: String,
	pub time: i32,
	pub source: String
}

#[derive(Insertable)]
#[diesel(table_name = trade)]
pub struct NewTrade<'a> {
    pub owner : &'a str,
    pub itemstring : &'a str,
    pub itemname  : &'a str,
    pub stacksize : &'a i32,
    pub quantity : &'a i32,
    pub price: &'a i32,
	pub otherplayer: &'a str,
	pub player: &'a str,
	pub time: &'a i32,
	pub source: &'a str
}

