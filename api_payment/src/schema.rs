table! {
    credit_cards (id) {
        id -> Int4,
        credit_card_fk -> Int4,
        balance -> Int4,
        intermediary -> Varchar,
    }
}

table! {
    payments (payment_id) {
        payment_id -> Int4,
        user_id -> Int4,
        credit_card_id -> Int4,
        payment_date -> Timestamp,
        amount -> Int4,
    }
}

joinable!(payments -> credit_cards (credit_card_id));

allow_tables_to_appear_in_same_query!(
    credit_cards,
    payments,
);
