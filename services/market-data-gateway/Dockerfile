FROM rust:1.75-slim AS builder

WORKDIR /build

COPY services/market-data-gateway/Cargo.toml Cargo.toml
COPY services/market-data-gateway/src src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /build/target/release/market-data-gateway /app/market-data-gateway

CMD ["/app/market-data-gateway"]
