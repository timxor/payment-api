extern crate diesel;
extern crate payment_api;

use payment_api::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use payment_api::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");
    
    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}, {}", user.first_name, user.last_name);
        println!("----------\n");
    }
}
