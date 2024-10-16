FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
  build-essential \
  rustup

RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack

WORKDIR /opt/rust_chat_app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY axum_backend axum_backend
COPY wasm_frontend wasm_frontend

RUN cd wasm_frontend && ~/.cargo/bin/wasm-pack build --target web
RUN cargo build --release
