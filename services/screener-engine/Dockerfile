# Multi-stage Rust build
# Stage 1: builder — rust:1.75-slim, cargo build --release
# Stage 2: runtime — debian:bookworm-slim, copy binary
# EXPOSE: health check port only (9001-9005)
# CMD: ["/app/screener-engine"]
