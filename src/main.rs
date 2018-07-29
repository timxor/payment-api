//#![feature(plugin)]
//#![plugin(rocket_codegen)]
//
//extern crate rocket;
//extern crate dotenv;
//#[macro_use] extern crate rocket_contrib;
//#[macro_use] extern crate diesel;
//extern crate payment_api;
//extern crate r2d2;
//extern crate r2d2_diesel;
//
//mod schema;
//use diesel::prelude::*;
//use payment_api::*;
//use payment_api::models::User;
//use rocket_contrib::{Json, Value};
//
////#[post("/", data = "<user>")]
////fn create(user: Json<User>, connection: db::Connection) -> Json<User> {
////    let insert = User {..user.into_inner() };
////    Json(User::create(insert, &connection))
////}
//
////fn new_user(first_name: String, last_name: String, user_name: String, cell_number: i32, 
////            cell_verified: bool, email: String, public_key: String, private_key: String, 
////            eth_address: String) -> User {
////    let conn = establish_connection();
////    
////    let new_user = create_user(&conn, first_name, last_name, user_name, cell_number,
////                                cell_verified, email, private_key, public_key, eth_address); 
//////    println!("Saved person: {}", new_user.first_name);
////    new_user
////}
//
////#[get("/")]
////fn read(connection: db::Connection) -> Json<Value> {
////    Json(json!(User::read(&connection)))
////}
//
////#[put("/<id>", data = "<user>")]
////fn update(id: i32, user: Json<User>, connection: db::Connection) -> Json<Value> {
////    
////    // todo fix this, update was not working, delete is not right
////    let update = User { id: Some(id), ..user.into_inner() };
////    Json(json!({
////        "success": User::delete(id, &connection)
////    }))
////}
////
////#[delete("/<id>")]
////fn delete(id: i32, connection: db::Connection) -> Json<Value> {
////    Json(json!({
////        "success": User::delete(id, &connection)
////    }))
////}
//
//fn main() {
////    rocket::ignite()
//////        .mount("/user", routes![update, delete])
//////        .mount("/users", routes![read])
////        .manage(db::connect())
////        .launch();
//}
