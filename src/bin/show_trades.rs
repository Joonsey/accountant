use accountant::models::*;
use diesel::prelude::*;
use accountant::*;

fn main()
{
    use accountant::schema::trade::dsl::*;

    let connection = &mut establish_connection();
    let results = trade
        .filter(source.like("Vendor"))
        .limit(5)
        .load::<Trade>(connection)
        .expect("error loading trades");

    println!("Displaying {} trades", results.len());
    for result in results {
        println!("sold by {}", result.player);
        println!("sold to {}.", result.source);
        println!("-----------\n");
    }
}
