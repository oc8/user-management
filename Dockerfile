FROM rust:alpine3.19

WORKDIR /app

RUN apk add --no-cache musl-dev

RUN cargo install cargo-watch