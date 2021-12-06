FROM rust:1.56

WORKDIR payment_ms/

RUN rustup default nightly

RUN apt update && apt install libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

WORKDIR api_payment/

RUN cargo build --release

CMD cargo run --release
