#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[macro_use] extern crate serde_derive;
use self::models::{NewTransfer, Transfer};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &PgConnection, amount: &str, currency: &str, to_name: &str, to_number: 
&str,
                   to_email: &str, complete: &bool, )
    -> Transfer {
    use schema::transfers;

    let new_transfer = NewTransfer {
        amount: amount,
        currency: currency,
        to_name: to_name,
        to_number: to_number,
        to_email: to_email,
        complete: complete,
    };

    diesel::insert_into(transfers::table)
        .values(&new_transfer)
        .get_result(conn)
        .expect("Error saving new post")
}
