use rocket::serde::{ json::Json };
use diesel::prelude::*;
use diesel::RunQueryDsl;

//use rocket::http::Status;

use crate::database;
use crate::models::credit_cards::*;
use crate::schema::credit_cards::dsl::*;

use rand::Rng;

    

///
///    La función index retorna una lista de tipo `JSON` con todas las tarjetas 
///    que se encuentran guardadas, de momento está con motivo de desarrollo, 
///    cuando el microservicio esté listo para producción retornará un `403`.
///
///     
#[get("/credit-cards")]
pub fn index() -> Json<Vec<CreditCard>> {
    let connection = database::database_connection();
    let results = credit_cards
        .load::<CreditCard>(&connection)
        .expect("Error al obtener las tarjetas");
    
    Json(results)
}



///
///    La función show retorna un Json la información de una tarjeta específica
///    a partir de su id
///
/// 
#[get("/credit-cards/<card_id>")]
pub fn show (card_id: i32) -> Json<Vec<CreditCard>> {

    use diesel::ExpressionMethods;
    use diesel::QueryDsl;

    let connection = database::database_connection();
    let card_info = credit_cards
        .filter(credit_card_fk.eq(card_id))
        .limit(1)
        .load::<CreditCard>(&connection)
        .expect("Error");
    
    Json(card_info)
}



///
///    La función create guarda en la DB una nueva tarjeta de credito a partir de
///    un Json de tipo NewCreditCard y retorna un Json de tipo CreditCard
///
#[post("/credit-cards", format = "json", data = "<card>")]
pub fn create (card: Json<NewCreditCard>)-> Json<CreditCard>{
    let conn = database::database_connection();

    let number = card.intermediary.to_string();
    let card_intermediary;
    if number.starts_with("4"){
        card_intermediary = "Visa";
    }
    else if number.starts_with("5") {
        card_intermediary = "MasterCard";
    }
    else{
        card_intermediary = "Otro";
    }

    // Genera un saldo aleatorio a la nueva tarjeta
    let mut rng = rand::thread_rng();
    let random_balance: i32 = rng.gen_range(500000..=700000);

    let new_card = (
        credit_card_fk.eq(card.credit_card_fk),
        balance.eq(random_balance),
        intermediary.eq(card_intermediary)
    );
    let insert = diesel::insert_into(credit_cards)
        .values(&new_card).get_result::<CreditCard>(&conn).expect("Error");
    
    Json(insert)
    

    // status::Custom<content::Json<&'static str>>
}


#[put("/credit-cards/<card_id>", format  = "json", data = "<card_info>")]
pub fn update (card_id: i32, card_info: Json<NewCreditCard>) -> Json<CreditCard> {
    let conn = database::database_connection();

    // Cuando se actualiza la información de una tarjeta, se vuelve
    // a generar un nuevo saldo para evitar la situación en la que se
    // actualiza el medio de pago pero el saldo permanece en 0

    let mut rng = rand::thread_rng();
    let random_new_balance: i32 = rng.gen_range(200000..=400000);

    let update_card = (
        credit_card_fk.eq(card_id),
        balance.eq(random_new_balance),
        intermediary.eq(card_info.intermediary.to_string())
    );

    let update = diesel::update(credit_cards
        .filter(credit_card_fk.eq(card_id)))
        .set(update_card)
        .get_result::<CreditCard>(&conn)
        .expect("Error");
    
    Json(update)
}