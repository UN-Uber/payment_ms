
// Rutas para la entidad CreditCards

pub mod credit_cards{

    use rocket::serde::{ json::Json };
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use rocket::http::Status;
    use rocket::response::{content, status};

    use crate::database;
    use crate::schema::credit_cards::dsl::*;
    use crate::models::*;

    // La función index lista todos las tarjetas que se encuentran guardadas,
    // de momento está con motivo de desarrollo, cuando el microservicio esté
    // listo para producción retornará un  403.

    #[get("/credit-cards")]
    pub fn index() -> Json<Vec<CreditCard>> {
        let connection = database::database_connection();
        let results = credit_cards
            .load::<CreditCard>(&connection)
            .expect("Error al obtener las tarjetas");
        
        Json(results)
    }

    // La función create guarda en la DB una nueva tarjeta de credito a partir de
    // un Json de tipo NewCreditCard y retorna un 201 Created si el registro se guardó
    // correctamente y un 400 en caso de error.


    #[post("/credit-cards", format = "json", data = "<card>")]
    pub fn create (card: Json<NewCreditCard>)-> Status{
        let conn = database::database_connection();
        let new_card = (
            user_card_id.eq(card.user_card_id),
            balance.eq(card.balance),
            intermediary.eq(card.intermediary.to_string())
        );
        let insert = diesel::insert_into(credit_cards)
            .values(&new_card);
        
        match insert.execute(&conn) {
            Ok(_) => Status::Created,
            Err(error) => {
                Status::BadRequest
            }
        }

        // status::Custom<content::Json<&'static str>>
    }

    #[put("/credit-cards/<card_id>", format  = "json", data = "<card_info>")]
    pub fn update (card_id: i32, card_info: Json<NewCreditCard>) -> Status {
        let conn = database::database_connection();
        let update_card = (
            user_card_id.eq(card_id),
            balance.eq(card_info.balance),
            intermediary.eq(card_info.intermediary.to_string())
        );

        let update = diesel::update(credit_cards.find(card_id))
            .set(update_card)
            .execute(&conn);
        
        match update {
            Ok(_) => Status::Accepted,
            Err(error) => {
                Status::BadRequest
            }
        }
    }
}