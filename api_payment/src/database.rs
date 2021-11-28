use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

///    
///    Conexión con la base de datos haciendo uso del módulo pg de
///    Diesel, mediante la variable de entorno DATABASE_URL del
///    archivo .env
/// 
/// 
pub fn database_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL no ha sido establecida en .env");

    PgConnection::establish(&database_url)
        .expect(&format!("Error al connectar a la DB: {}", database_url))
}