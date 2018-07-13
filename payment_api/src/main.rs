#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket_contrib::{Json, Value};
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[cfg(test)] mod tests;



#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
mod db;
mod schema;
mod user;
use user::{User};

#[get("/")]
fn read() -> Json<Value> {
    Json(json!([
        "user 1", 
        "user 2"
    ]))
}
#[post("/", data = "<user>")]
fn create(user: Json<User>) -> Json<User> {
    user
}

#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>) -> Json<User> {
    user
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value> {
    Json(json!({"status": "ok"}))
}


fn main() {
    rocket::ignite()
        .mount("/user", routes![create, update, delete])
        .mount("/users", routes![read])
        .manage(db::connect())
        .launch();
}

