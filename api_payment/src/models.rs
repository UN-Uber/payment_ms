use crate::schema::*;
use rocket::serde::{ Serialize, Deserialize };
use diesel::{ Insertable, Queryable};

#[derive(Serialize, Queryable, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreditCard {
    pub id: i32,
    pub user_card_id: i32,
    pub balance: i32,
    pub intermediary: String
}

#[derive(Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub id: i32,
    pub user_id: i32,
    pub payment_date: chrono::DateTime<chrono::Utc>,
    pub amount: i32,
    pub credit_card_id: i32
}

#[derive(Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "credit_cards"]
pub struct NewCreditCard {
    pub user_card_id: i32,
    pub balance: i32,
    pub intermediary: String 
}

#[derive(Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "payments"]
pub struct NewPayment {
    pub user_id: i32,
    pub payment_date: chrono::NaiveDateTime,
    pub amount: i32,
    pub credit_card_id: i32 
}