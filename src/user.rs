use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use schema::users;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub cell_number: String,
    pub cell_verified: bool,
    pub email: String,
    pub public_key: String,
    pub private_key: String,
    pub eth_address: String
}


#[table_name = "users"]
#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub cell_number: String,
    pub cell_verified: bool,
    pub email: String,
    pub public_key: String,
    pub private_key: String,
    pub eth_address: String
}





//#[derive(Insertable)]
//#[table_name="users"]
//pub struct NewPerson<'a> {
//    pub id: Uuid,
//    pub name: &'a str,
//}


// this is called in main.rs create() function
impl User {
    pub fn create(user: User, connection: &PgConnection) -> User {
        
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");

        users::table.order(users::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &PgConnection) -> Vec<User> {
        users::table.order(users::id.asc())
            .load::<User>(connection)
            .unwrap()
    }

    pub fn update(id: i32, user: User, connection: &PgConnection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
    }
    
    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(users::table.find(id))
            .execute(connection)
            .is_ok()
    }
}

//        println!("\n\n -----------------------------------------------");
//        println!("1      user.create()       ");
//        println!("-----------------------------------------------");
//
//        // execute a cli to generate a key pair
//        let output = Command::new("/Users/tim.siwula/Desktop/parity/target/debug/./ethkey")
//            .arg("generate")
//            .arg("random")
//            .output()
//            .expect("./ethkey failed to execute");
//        assert!(output.status.success());
//
//        let mut data = String::from_utf8_lossy(&output.stdout);
//        let mut secret = String::new();
//        secret = data[9..73].to_string();
//        println!("\nsecret = {:?}", secret);
//
//        let mut public = String::new();
//        public = data[83..211].to_string();
//        println!("\npublic = {:?}", public);
//
//
//        let mut address: String = "0x".to_owned();
//        let borrowed_string: &str = &data[221..261];
//
//        address.push_str(borrowed_string);
//
//        println!("\naddress = {:?}", address);
//        
//        
//        &user.pubKey = public;
//        &user.privKey = secret;
//        &user.address = address;
//
//
//        println!("\n\n -----------------------------------------------");
//        println!("2      post assignment       ");
//        println!("-----------------------------------------------");
