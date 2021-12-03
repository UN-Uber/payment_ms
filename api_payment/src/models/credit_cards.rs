use crate::schema::credit_cards;
use rocket::serde::{ Serialize, Deserialize };
use diesel::{Insertable, Queryable };


#[derive(Serialize, Queryable, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreditCard {
    pub id: i32,
    pub user_card_id: i32,
    pub balance: i32,
    pub intermediary: String
}


#[derive(Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "credit_cards"]
pub struct NewCreditCard {
    pub user_card_id: i32,
    pub balance: Option<i32>,
    pub intermediary: String 
}