#!/bin/bash
# VPS Setup Script for NeuralTrade
# Steps:
#   1. Install Docker + Docker Compose
#   2. Install Rust toolchain
#   3. Install Java 21 (GraalVM)
#   4. Install Python 3.11 + pip packages
#   5. Create TimescaleDB + run init.sql
#   6. Create Kafka topics (topics.sh)
#   7. Configure firewall (only port 80/443/22 open)
#   8. Setup SSL with Certbot
#   9. Configure systemd services for auto-restart
#   10. Run data loader (load_historical.py)
#   11. Start all services via docker-compose
#   12. Verify health endpoints for all services
