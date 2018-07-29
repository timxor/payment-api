extern crate diesel;
extern crate payment_api;

use payment_api::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use payment_api::schema::users::dsl::*;

    let connection = establish_connection();
    println!("-----------------------------------------------------------------------------------");
    println!("  [first_name], [last_name], [username], [email], [eth_address]");
    println!("-----------------------------------------------------------------------------------");

    let results = users
        .limit(20)
        .load::<User>(&connection)
        .expect("Error loading users");

    for user in results {
        println!("  {}, {}, {}, {}, {}", user.first_name, user.last_name, user.user_name, user.email, user.eth_address);
        println!("-----------------------------------------------------------------------------------");
    }
}
