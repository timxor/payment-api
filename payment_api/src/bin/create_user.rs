extern crate diesel;
extern crate payment_api;
use std::process::Command;
use payment_api::*;

fn main() {
    let connection = establish_connection();

    // execute a cli to generate a key pair
    let output = Command::new("/Users/tim.siwula/Desktop/parity/target/debug/./ethkey")
        .arg("generate")
        .arg("random")
        .output()
        .expect("./ethkey failed to execute");
    assert!(output.status.success());
    
    let mut data = String::from_utf8_lossy(&output.stdout);

    for s in data.lines() {
        println!("{:?}", s);
    }
    
    // todo save secret, public and address to postgresql db
    
    let mut secret = String::new();
    secret = data[9..73].to_string();
    println!("\nsecret = {:?}", secret);

    let mut public = String::new();
    public = data[83..211].to_string();
    println!("\npublic = {:?}", public);


    let mut address: String = "0x".to_owned();
    let borrowed_string: &str = &data[221..261];

    address.push_str(borrowed_string);

    println!("\naddress = {:?}", address);
    
   
   // todo finish creating function
   // let user = create_user(&connection, secret, public, address);
}
