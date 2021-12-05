CREATE TABLE credit_cards (
    id SERIAL PRIMARY KEY,
    credit_card_fk INTEGER UNIQUE NOT NULL,
    balance INTEGER NOT NULL,
    intermediary VARCHAR(20) NOT NULL
);