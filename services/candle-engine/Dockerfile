FROM rust:1.75-slim AS builder

WORKDIR /build

COPY services/candle-engine/Cargo.toml Cargo.toml
COPY services/candle-engine/src src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /build/target/release/candle-engine /app/candle-engine

CMD ["/app/candle-engine"]
