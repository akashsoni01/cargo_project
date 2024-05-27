FROM rust:latest

WORKDIR /usr/src/poc_hello_world

COPY . .

RUN cargo build --release

CMD cargo run

