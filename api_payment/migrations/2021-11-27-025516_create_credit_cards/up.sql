CREATE TABLE credit_cards (
    id SERIAL PRIMARY KEY,
    user_card_id INTEGER NOT NULL,
    balance INTEGER NOT NULL,
    intermediary VARCHAR(20) NOT NULL
);