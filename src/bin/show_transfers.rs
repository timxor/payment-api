extern crate diesel;
extern crate payment_api;

use payment_api::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::transfers::dsl::*;

    let connection = establish_connection();
    let results = transfers
        .filter(complete.eq(false))
        .limit(5)
        .load::<Transfer>(&connection)
        .expect("Error loading transfers");

    println!("Displaying {} transfers", results.len());
    let mut index = 1;
    for transfer in results {

        println!("\n++++++ transfer id: {} +++++ transfer complete: {} +++++", index, transfer
            .complete);
        index = index + 1;

        println!("---------------------------------");
        println!("amount: {} {}", transfer.amount, transfer.currency);
        println!("---------------------------------");

        println!("recipient: {}, {}, {},", transfer.to_name, transfer.to_number, transfer.to_email);
        println!("---------------------------------");
    }
}
