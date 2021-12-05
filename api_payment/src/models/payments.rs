use crate::schema::payments;
use rocket::serde::{ Serialize, Deserialize };
use diesel::{Insertable, Queryable };
use chrono::{ NaiveDateTime };


#[derive(Serialize, Queryable, AsChangeset)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub payment_id: i32,
    pub user_id: i32,
    pub credit_card_id: i32,
    pub payment_date: NaiveDateTime,
    pub amount: i32,
}


#[derive(Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "payments"]
pub struct NewPayment {
    pub user_id: i32,
    pub payment_date: Option<NaiveDateTime>,
    pub amount: i32,
    pub credit_card_id: i32 
}