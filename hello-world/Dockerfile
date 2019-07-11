FROM rust:1.36.0-slim-stretch as build

WORKDIR ./build
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./src ./src

RUN cargo test && \
    cargo build --release && \
    mkdir ./artifacts && \
    cp ./target/release/hello-world ./artifacts/

FROM debian:stretch-slim

WORKDIR ./app
COPY --from=build /build/artifacts/hello-world .

CMD ["/app/hello-world"]
