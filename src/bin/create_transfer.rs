extern crate diesel;
extern crate payment_api;

use payment_api::*;
use std::io::{stdin};

fn main() {
    let connection = establish_connection();

    let mut amount = String::new();
    let mut currency = String::new();
    let mut name = String::new();
    let mut number = String::new();
    let mut email = String::new();
    let complete = &false;


    println!("How much do you want to transfer?");
    stdin().read_line(&mut amount).unwrap();
    let amount = amount.trim_right(); // Remove the trailing newline


    println!("What token do you want to use?");
    stdin().read_line(&mut currency).unwrap();
    let currency = currency.trim_right(); // Remove the trailing newline
    
    println!("what is their name?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_right(); // Remove the trailing newline


    println!("what is their number?");
    stdin().read_line(&mut number).unwrap();
    let number = number.trim_right(); // Remove the trailing newline


    println!("what is their email?");
    stdin().read_line(&mut email).unwrap();
    let email = email.trim_right(); // Remove the trailing newline

    
//    let transfer = create_transfer(&connection, amount, &currency, name, number, email, complete);
    println!("\n~~~~ Sent transfer ~~~~~");
    println!("Amount: {} {}", amount, currency);
    println!("recipient: {}, {}, {}", name, number, email);
   // println!("transaction id: {}\n", transfer.id);
}
