# NeuralTrade — AI-Powered Stock Analysis Platform

## Quick Start
1. Copy `.env.example` to `.env` and fill credentials
2. `docker-compose up timescaledb redis kafka`
3. `python scripts/data-loader/load_historical.py`
4. Start services in order (see claude.md Development Order)
5. `cd frontend && npm run dev`

## Full Documentation
See `claude.md` for complete system requirements, architecture,
service specifications, and development guide.

## Services
| Service | Language | Port |
|---------|----------|------|
| market-data-gateway | Rust | internal |
| candle-engine | Rust | internal |
| indicator-engine | Rust | internal |
| screener-engine | Rust | internal |
| scoreboard-engine | Rust | internal |
| ml-service | Python | 8001 |
| story-engine | Java | 8002 |
| ai-validation | Java | 8003 |
| signal-service | Java | 8004 |
| alert-service | Java | 8005 |
| api-gateway | Java | 8080 |
| auth-service | Java | 8006 |
| subscription-service | Java | 8007 |
| frontend | Next.js | 3000 |
