FROM rust:1.75-slim AS builder

WORKDIR /build

COPY services/scoreboard-engine/Cargo.toml Cargo.toml
COPY services/scoreboard-engine/src src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /build/target/release/scoreboard-engine /app/scoreboard-engine

CMD ["/app/scoreboard-engine"]
