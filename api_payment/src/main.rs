#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod controllers;
mod database;
mod models;
mod schema;

#[get("/")]
pub fn hello () -> &'static str {
    "Raiz de la API de Payments"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![

        hello,

        //Rutas para CreditCards
        controllers::credit_cards_controller::index,
        controllers::credit_cards_controller::show,
        controllers::credit_cards_controller::create,
        controllers::credit_cards_controller::update,

        //Rutas para Payments
        controllers::payments_controller::index,
        controllers::payments_controller::show_payment,
        controllers::payments_controller::create
    ])
}