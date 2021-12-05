use rocket::serde::{ json::Json };
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::dsl;
use rocket::http::Status;

use crate::database;
use crate::schema::payments::dsl::*;
use crate::models::payments::*;

use crate::models::credit_cards::*;
use crate::schema::credit_cards::dsl::*;



///
///    La función index lista todos los pagos que se hallan guardado,
///    de momento está con motivo de desarrollo, cuando el microservicio esté
///    listo para producción retornará un `403`.
///
#[get("/payments")]
pub fn index() -> Json<Vec<Payment>> {
    let connection = database::database_connection();
    let results = payments
        .order(payment_date.desc())
        .load::<Payment>(&connection)
        .expect("Error");
    
    Json(results)
}



///
///    La función show_payment muestra los pagos asociados a un usuario
///    a partir de su id, retorna un `JSON` con la información correspondiente
///    a cada uno de los pagos ordenados con más reciente primero
/// 
///
#[get("/users/<number>/payments")]
pub fn show_payment(number: i32) -> Json<Vec<Payment>> {
    let connection = database::database_connection();
    let results = payments
        .filter(user_id.eq(number))
        .limit(20)
        .order(payment_date.desc())
        .load::<Payment>(&connection)
        .expect("Error");
    
    Json(results)
}



///
///    La función create guarda en la base de datos la información de un nuevo pago
///    a partir de un `JSON` de tipo NewPayment y retorna un Json de tipo Payment con
///    los datos insertados en la base de datos
/// 
#[post("/payments", format = "json", data = "<payment_info>")]
pub fn create (payment_info: Json<NewPayment>) -> Json<Payment> {
    let connection = database::database_connection();
    let new_payment = (
        user_id.eq(payment_info.user_id),
        payment_date.eq(dsl::now),
        amount.eq(payment_info.amount),
        credit_card_id.eq(payment_info.credit_card_id)            
    );

    let mut card_info = credit_cards
        .filter(credit_card_fk.eq(payment_info.credit_card_id))
        .limit(1)
        .load::<CreditCard>(&connection)
        .expect("Error");

    card_info[0].balance = card_info[0].balance - payment_info.amount;

    let _update_balance = diesel::update(credit_cards.find(card_info[0].id))
        .set(balance.eq(card_info[0].balance))
        .execute(&connection);


    let insert: Payment = diesel::insert_into(payments).values(&new_payment).get_result::<Payment>(&connection).expect("Error");

    Json(insert)
}