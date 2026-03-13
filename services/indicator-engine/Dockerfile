FROM rust:1.75-slim AS builder

WORKDIR /build

COPY services/indicator-engine/Cargo.toml Cargo.toml
COPY services/indicator-engine/src src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /build/target/release/indicator-engine /app/indicator-engine

CMD ["/app/indicator-engine"]
