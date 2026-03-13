# NeuralTrade

NeuralTrade is a reference implementation of the platform described in [instructions.md](instructions.md): a multi-service Indian equities analysis stack for Indian equities with a product UI, scoring pipeline, ML service, and business-service layer.

## Default Startup

The project is now configured so the full stack can be started with:

```bash
docker compose up -d
```

This command brings up:

- TimescaleDB
- Redis
- Kafka
- `market-data-gateway`
- `candle-engine`
- `indicator-engine`
- `screener-engine`
- `scoreboard-engine`
- `ml-service`
- `story-engine`
- `ai-validation`
- `signal-service`
- `alert-service`
- `auth-service`
- `subscription-service`
- `api-gateway`
- `frontend`

## Crash-Free Config Handling

External integrations are treated as optional at startup.

If broker keys, Claude keys, Telegram tokens, Razorpay keys, or other external configuration values are not set:

- the stack still starts
- the affected service stays up
- the runtime returns a clear message:
  `Configuration not set for this operation.`
- the frontend reference page also shows which config-dependent operations are unavailable

This behavior is implemented through:

- `scripts/runtime/reference_service.py`
- `scripts/runtime/reference_frontend.py`
- the default `docker-compose.yml` service definitions

## Compose Runtime Mode

The default Compose path is intentionally resilient.

- Infrastructure services run as real containers.
- Application services run in a config-aware reference runtime so `docker compose up -d` does not depend on local Rust, Maven, npm, or pip builds.
- The source code for the real service implementations is still present in the repo for manual development.

## Repository Layout

| Path | Purpose |
|------|---------|
| `frontend/` | Next.js UI source |
| `services/market-data-gateway` | Rust ingest reference implementation |
| `services/candle-engine` | Rust candle aggregation reference implementation |
| `services/indicator-engine` | Rust indicator calculation reference implementation |
| `services/screener-engine` | Rust screener reference implementation |
| `services/scoreboard-engine` | Rust scoring reference implementation |
| `services/ml-service` | Python ML reference implementation |
| `services/story-engine` | Java story service logic |
| `services/ai-validation` | Java validation service logic |
| `services/signal-service` | Java signal lifecycle logic |
| `services/alert-service` | Java alert logic |
| `services/api-gateway` | Java controller-layer logic |
| `services/auth-service` | Java auth logic |
| `services/subscription-service` | Java subscription logic |
| `scripts/runtime/` | Compose-safe reference runtimes |

## Manual Development

If you want to work on the actual implementation code instead of the Compose-safe runtime, use the service folders directly.

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

### Rust Services

Example:

```bash
cd services/indicator-engine
cargo run
```

### Java Services

The Java folders contain implemented service classes and valid Maven descriptors, but they are still reference business-logic modules rather than fully wired Spring Boot applications.

## Verification

Validated in this environment:

- `docker compose config`
- `python -m compileall services/ml-service/src`

Not executed here:

- full Docker image builds
- `cargo build`
- Maven compilation
- frontend npm build

## Notes

- `instructions.md` remains the product specification source of truth.
- The default Compose experience prioritizes startup reliability and graceful degradation when external APIs are not configured.
- The service source code remains in place for future progression from reference runtime to full production wiring.
