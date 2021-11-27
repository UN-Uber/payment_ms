pub mod credit_cards{
    use rocket::serde::json::Json;
    use diesel::prelude::*;

    use crate::database;
    use crate::schema::credit_cards::dsl::*;
    use crate::schema::payments::dsl::*;
    use crate::models::*;

    #[get("/credit-cards")]
    pub fn index() -> Json<Vec<CreditCard>> {
        let connection = database::database_connection();
        let results = credit_cards
            .load::<CreditCard>(&connection)
            .expect("Error al obtener las tarjetas");
        
        Json(results)
    }
}