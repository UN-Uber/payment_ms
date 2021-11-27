CREATE TABLE payments (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    credit_card_id INTEGER NOT NULL,
    payment_date TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    amount INTEGER NOT NULL,
    CONSTRAINT credit_card_fk
        FOREIGN KEY (credit_card_id)
        REFERENCES credit_cards(id)
        ON DELETE CASCADE
);