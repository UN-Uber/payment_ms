#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;
extern crate serde;
extern crate serde_json;

pub mod controllers;
pub mod database;
pub mod models;
pub mod schema;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        controllers::credit_cards::index
    ])
}