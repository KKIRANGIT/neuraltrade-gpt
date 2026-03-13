# NeuralTrade

NeuralTrade is a reference implementation of the platform described in [instructions.md](instructions.md): a multi-service Indian equities analysis stack with a Next.js product UI, Rust market pipeline, Python ML service, and Java business services.

This repository now contains:

- A real `frontend/` Next.js 14 app with landing, dashboard, stocks, stock detail, signals, auth, and settings pages.
- Seeded domain data for scoreboard, stories, signals, market context, and subscription plans.
- Rust reference implementations for:
  - `market-data-gateway`
  - `candle-engine`
  - `indicator-engine`
  - `screener-engine`
  - `scoreboard-engine`
- Python reference implementation for `ml-service` with deterministic feature building, model heuristics, batch inference, retraining report generation, and FastAPI endpoints.
- Java reference implementations for story generation, AI validation, signal lifecycle, alerts, auth, subscriptions, and API gateway controller surfaces.

## Repository Layout

| Path | Purpose |
|------|---------|
| `frontend/` | Product UI built with Next.js 14 + Tailwind |
| `services/market-data-gateway` | Tick ingestion, broker mocks, ring buffer, batching |
| `services/candle-engine` | Multi-timeframe candle building and storage |
| `services/indicator-engine` | Indicator snapshot calculation and confluence scoring |
| `services/screener-engine` | 30-screener style rule evaluation and dedup |
| `services/scoreboard-engine` | Bull/bear scoring, weighting, eligibility tiers |
| `services/ml-service` | Feature builder, heuristic models, batch inference API |
| `services/story-engine` | Rule-based English and Telugu story generation |
| `services/ai-validation` | Claude-style validation flow and prompt building |
| `services/signal-service` | Signal lifecycle and accuracy tracking |
| `services/alert-service` | Telegram alert orchestration |
| `services/api-gateway` | Stock and signal controller layer |
| `services/auth-service` | Registration/login/refresh/logout logic |
| `services/subscription-service` | Plan access and subscription handling |

## Frontend

The frontend is the most complete runnable surface in the repo.

Implemented pages:

- `/`
- `/dashboard`
- `/stocks`
- `/stocks/[symbol]`
- `/signals`
- `/settings`
- `/auth`

Implemented UI features:

- Market regime banner
- Top bull and bear scorecards
- Multi-timeframe scoreboard widget
- Signal cards with entry/stop/target ladder
- Telugu and English story rendering
- Subscription and profile views

## Service Status

These services are implemented as reference code focused on domain logic rather than live infrastructure integration:

- Rust services use deterministic/mock inputs instead of live Kafka, Redis, or broker sockets.
- Java services are implemented as in-memory business logic classes rather than full deployed Spring Boot apps.
- Python ML service exposes concrete logic and can be compiled locally; it uses heuristic models instead of trained production artifacts.

This is intentional so the repository remains self-contained and readable while still covering the complete product flow described in the requirements.

## Local Development

### Frontend

```bash
cd frontend
npm install
npm run dev
```

### Python ML Service

```bash
cd services/ml-service/src
uvicorn main:app --reload --port 8001
```

### Rust Reference Services

Each Rust service now has a real `Cargo.toml`. Example:

```bash
cd services/indicator-engine
cargo run
```

### Java Reference Services

The Java folders contain implemented service classes and controller surfaces, but they are currently structured as reference business logic and not yet wired into full Spring Boot applications.

## Verification

Completed during this implementation:

- `python -m compileall services/ml-service/src`

Not completed in this environment:

- `npm install` / `npm run build` for the frontend
- `cargo build` for Rust services
- Maven builds for Java services

Those steps were not run because dependency installation and full multi-language builds were not available in the current restricted environment.

## Notes

- `instructions.md` remains the product specification source of truth.
- The current implementation is a coherent platform reference build, not a live brokerage-connected trading system.
- Infrastructure files such as Docker, TimescaleDB, Redis, and Kafka scaffolding are still present for future wiring.
