// SignalService
// Consumes validated signals from Kafka: signals.validated
// Stores in TimescaleDB table: signals
// Manages signal lifecycle: ACTIVE → T1_HIT / T2_HIT / T3_HIT / STOPPED / EXPIRED
// Price monitoring: subscribes to live price feed, checks target/stop hit
// On target hit: update status, publish to Kafka: signals.updates
// On stop hit: update status, record loss, publish update
// On expiry: mark expired if valid_until passed and still ACTIVE
// Provides REST endpoints for frontend signal dashboard
