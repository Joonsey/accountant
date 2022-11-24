use accountant::*;
use std::io::{stdin, Read};

fn main()
{
    let connection = &mut establish_connection();

    let mut itemname = String::new();

    println!("Enter item");
    stdin().read_line(&mut itemname).unwrap();
    let itemstring = "";
    let itemname = itemname.trim_end();
    let stacksize: i64 = 1;
    let quantity: i64 = 1;
    let price: i64  = 1;
    let otherplayer = "Generous beneficiairy";
    let player = "";
    let time: i64 = 1;
    let source = "";
    let post = create_trade(
        connection,
        &itemstring,
        &itemname,
        &stacksize,
        &quantity,
        &price,
        &otherplayer,
        &player,
        &time,
        &source
        );

    println!(
        "\nSaved draft for itemname: {}, with id {}",
        itemname, post.id);
}
