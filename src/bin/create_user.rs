extern crate diesel;
extern crate payment_api;
use std::process::Command;
use payment_api::*;
use std::io::{stdin, Read};

fn main() {
    let access = false;
    create_new_user(access);
}

fn create_new_user(api: bool) {
    let connection = establish_connection();

    if(api) {
        println!("create_new_user *** api access");

    } else {
        println!("create_new_user *** cli access");
        
        let mut first_name = String::new();
        println!("What would you like your first_name to be?");
        stdin().read_line(&mut first_name).unwrap();
        let first_name = first_name.trim_right();

        let mut last_name = String::new();
        last_name = "lastname".to_string();

        let mut user_name = String::new();
        user_name = "username".to_string();

        let mut email = String::new();
        email = "you@gmail.com".to_string();

        // execute a cli to generate a key pair
        let output = Command::new("/Users/tim.siwula/Desktop/parity/target/debug/./ethkey")
            .arg("generate")
            .arg("random")
            .output()
            .expect("./ethkey failed to execute");
        assert!(output.status.success());

        let data = String::from_utf8_lossy(&output.stdout);

        let mut private_key = String::new();
        private_key = data[9..73].to_string();
        //println!("\nprivate_key = {:?}", private_key);

        let mut public_key = String::new();
        public_key = data[83..211].to_string();
        //println!("\npublic_key = {:?}", public_key);


        let mut eth_address: String = "0x".to_owned();
        let borrowed_string: &str = &data[221..261];

        eth_address.push_str(borrowed_string);

        //println!("\neth_address = {:?}", eth_address);

        let user = create_user(&connection, &first_name, &last_name, &user_name, &email, &public_key, &private_key, &eth_address);
        println!("\n Created user {}, {}", first_name, eth_address);
    }
}

