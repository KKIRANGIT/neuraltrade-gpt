FROM rust:1.75-slim AS builder

WORKDIR /build

COPY services/screener-engine/Cargo.toml Cargo.toml
COPY services/screener-engine/src src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /build/target/release/screener-engine /app/screener-engine

CMD ["/app/screener-engine"]
