use rocket::serde::{ json::Json };
    use diesel::prelude::*;
    use diesel::RunQueryDsl;
    use diesel::dsl;

    use rocket::http::Status;

    use crate::database;
    use crate::schema::payments::dsl::*;
    use crate::models::payments::*;



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