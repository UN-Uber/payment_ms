
/// 
///   ### Rutas para la entidad CreditCards
///

pub mod credit_cards{

    use rocket::serde::{ json::Json };
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use rocket::http::Status;

    use crate::database;
    use crate::schema::credit_cards::dsl::*;
    use crate::models::*;

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
            .filter(user_card_id.eq(card_id))
            .limit(1)
            .load::<CreditCard>(&connection)
            .expect("Error");
        
        Json(card_info)
    }


    
    ///
    ///    La función create guarda en la DB una nueva tarjeta de credito a partir de
    ///    un Json de tipo NewCreditCard y retorna un 201 Created si el registro se guardó
    ///    correctamente y un 400 en caso de error.
    ///
    #[post("/credit-cards", format = "json", data = "<card>")]
    pub fn create (card: Json<NewCreditCard>)-> Status{
        let conn = database::database_connection();

        // Genera un saldo aleatorio a la nueva tarjeta
        let mut rng = rand::thread_rng();
        let random_balance: i32 = rng.gen_range(60000..=100000);

        let new_card = (
            user_card_id.eq(card.user_card_id),
            balance.eq(random_balance),
            intermediary.eq(card.intermediary.to_string())
        );
        let insert = diesel::insert_into(credit_cards)
            .values(&new_card);
        
        match insert.execute(&conn) {
            Ok(_) => Status::Created,
            Err(_) => {
                Status::BadRequest
            }
        }

        // status::Custom<content::Json<&'static str>>
    }


    #[put("/credit-cards/<card_id>", format  = "json", data = "<card_info>")]
    pub fn update (card_id: i32, card_info: Json<NewCreditCard>) -> Status {
        let conn = database::database_connection();

        // Cuando se actualiza la información de una tarjeta, se vuelve
        // a generar un nuevo saldo para evitar la situación en la que se
        // actualiza el medio de pago pero el saldo permanece en 0

        let mut rng = rand::thread_rng();
        let random_new_balance: i32 = rng.gen_range(60000..=100000);

        let update_card = (
            user_card_id.eq(card_id),
            balance.eq(random_new_balance),
            intermediary.eq(card_info.intermediary.to_string())
        );

        let update = diesel::update(credit_cards
            .filter(user_card_id.eq(card_id)))
            .set(update_card)
            .execute(&conn);
        
        match update {
            Ok(_) => Status::Accepted,
            Err(_) => {
                Status::BadRequest
            }
        }
    }
}



///
///   ### Rutas para la entidad Payments
///

pub mod payments {
    use rocket::serde::{ json::Json };
    use diesel::prelude::*;
    use diesel::RunQueryDsl;
    use diesel::dsl;

    use rocket::http::Status;

    use crate::database;
    use crate::schema::payments::dsl::*;
    use crate::models::*;



    ///
    ///    La función index lista todos los pagos que se hallan guardado,
    ///    de momento está con motivo de desarrollo, cuando el microservicio esté
    ///    listo para producción retornará un `403`.
    ///
    #[get("/payments")]
    pub fn index() -> Json<Vec<Payment>> {
        let connection = database::database_connection();
        let results = payments
            .load::<Payment>(&connection)
            .expect("");
        
        Json(results)
    }



    ///
    ///    La función show_payment muestra los pagos asociados a un usuario
    ///    a partir de su id, retorna un `JSON` con la información correspondiente
    ///    a cada uno de los pagos 
    /// 
    ///
    #[get("/users/<number>/payments")]
    pub fn show_payment(number: i32) -> Json<Vec<Payment>> {
        let connection = database::database_connection();
        let results = payments
            .filter(user_id.eq(number))
            .limit(10)
            .load::<Payment>(&connection)
            .expect("Error");
        
        Json(results)
    }



    ///
    ///    La función create guarda en la base de datos la información de un nuevo pago
    ///    a partir de un `JSON` de tipo NewPayment y retorna un código `201 Created` si
    ///    este se guardo satisfactoriamente, en caso contrario devuelve un `400 Bad Request`
    /// 
    #[post("/payments", format = "json", data = "<payment_info>")]
    pub fn create (payment_info: Json<NewPayment>) -> Status {
        let connection = database::database_connection();
        let new_payment = (
            user_id.eq(payment_info.user_id),
            payment_date.eq(dsl::now),
            amount.eq(payment_info.amount),
            credit_card_id.eq(payment_info.credit_card_id)            
        );

        let insert = diesel::insert_into(payments).values(new_payment);

        match insert.execute(&connection) {
            Ok(_) => Status::Created,
            Err(_) => {
                Status::BadRequest
            }
        }
    }
}