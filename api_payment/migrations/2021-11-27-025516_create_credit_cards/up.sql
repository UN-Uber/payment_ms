CREATE TABLE credit_cards (
    id SERIAL PRIMARY KEY,
    user_card_id INTEGER UNIQUE NOT NULL,
    balance INTEGER NOT NULL,
    intermediary VARCHAR(20) NOT NULL
);