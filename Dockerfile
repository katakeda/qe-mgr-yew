FROM rust:latest as builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
WORKDIR /usr/src/qe-mgr
COPY . .
RUN cd backend && cargo build --release
RUN trunk build --release

FROM debian:stable-slim
RUN mkdir -p /usr/local/bin
COPY --from=builder /usr/src/qe-mgr/backend/target/release/qe-mgr /usr/local/bin/qe-mgr
COPY --from=builder /usr/src/qe-mgr/dist /usr/local/bin/dist
WORKDIR /usr/local/bin
CMD ["qe-mgr"]
