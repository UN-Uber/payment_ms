use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn database_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL no ha sido establecida en .env");

    PgConnection::establish(&database_url)
        .expect(&format!("Error al connectar a la DB: {}", database_url))
}