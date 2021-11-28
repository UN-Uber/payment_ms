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

        //Rutas para CreditCards
        controllers::credit_cards::index,
        controllers::credit_cards::show,
        controllers::credit_cards::create,
        controllers::credit_cards::update,

        //Rutas para Payments
        controllers::payments::index,
        controllers::payments::show_payment,
        controllers::payments::create
    ])
}