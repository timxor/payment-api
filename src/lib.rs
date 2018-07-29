#[macro_use] extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod schema;
pub mod models;

use self::models::{NewUser, User};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::insert_into;
use diesel::pg::Pg;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// http://diesel.rs/guides/getting-started/
// https://github.com/diesel-rs/diesel/blob/v1.3.0/examples/postgres/getting_started_step_3/src/lib.rs

pub fn create_user<'a>(conn: &PgConnection, first_name: &'a str, last_name: &'a str,
                    user_name: &'a str, email: &'a str, public_key: &'a str,
                    private_key: &'a str, eth_address: &'a str) -> User  {
    use schema::users;
    
    let new_user = NewUser {
        first_name: first_name,
        last_name: last_name,
        user_name: user_name,
        email: email,
        public_key: public_key,
        private_key: private_key,
        eth_address: eth_address,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
    
}
