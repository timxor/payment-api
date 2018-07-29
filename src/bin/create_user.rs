//extern crate diesel;
//extern crate payment_api;
//use std::process::Command;
//use payment_api::*;
//
//fn main() {
//    let connection = establish_connection();
//
//    // execute a cli to generate a key pair
//    let output = Command::new("/Users/tim.siwula/Desktop/parity/target/debug/./ethkey")
//        .arg("generate")
//        .arg("random")
//        .output()
//        .expect("./ethkey failed to execute");
//    assert!(output.status.success());
//    
//    let data = String::from_utf8_lossy(&output.stdout);
//
////    for s in data.lines() {
////        println!("{:?}", s);
////    }
//    
//    // todo save secret, public and address to postgresql db
//    
//    let mut secret = String::new();
//    secret = data[9..73].to_string();
//    println!("\nsecret = {:?}", secret);
//
//    let mut public = String::new();
//    public = data[83..211].to_string();
//    println!("\npublic = {:?}", public);
//
//
//    let mut address: String = "0x".to_owned();
//    let borrowed_string: &str = &data[221..261];
//
//    address.push_str(borrowed_string);
//
//    println!("\naddress = {:?}", address);
//    
//   
//   // todo finish creating function
//   // let user = create_user(&connection, secret, public, address);
//}
extern crate diesel;
extern crate payment_api;

use payment_api::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    let mut first_name = String::new();
    println!("What would you like your first_name to be?");
    stdin().read_line(&mut first_name).unwrap();
    let first_name = first_name.trim_right(); // Remove the trailing newline

    let mut last_name = String::new();
    last_name = "wow".to_string();

    let mut user_name = String::new();
    user_name = "wow".to_string();
    let mut email = String::new();
    email = "wow".to_string();
    let mut public_key = String::new();
    public_key = "wow".to_string();
    let mut private_key = String::new();
    private_key = "wow".to_string();

    let mut eth_address = String::new();
    eth_address = "wow".to_string();
    

    let user = create_user(&connection, &first_name, &last_name, &user_name, &email, &public_key, &private_key, &eth_address);
    println!("\nSaved draft {}", first_name);
    
    
    
    
    
    
    
}

