FROM rust:1.56

WORKDIR payment_ms/

RUN rustup default nightly

RUN apt update && apt install libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

WORKDIR api_payment/

RUN cargo build --release

#RUN echo DATABASE_URL=postgres://payment:password@payment_db/payment_db > .env

EXPOSE 8000

CMD ["cargo", "run", "--release"]
