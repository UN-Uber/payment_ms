FROM rust:1.57

WORKDIR payment_ms/

RUN apt update && apt install libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

#RUN git clone https://github.com/vishnubob/wait-for-it.git

WORKDIR api_payment/

CMD cargo run --release