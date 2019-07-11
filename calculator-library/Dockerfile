FROM rust:1.36.0-slim-stretch

WORKDIR ./build
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./src ./src

RUN cargo test && \
    cargo build --release
