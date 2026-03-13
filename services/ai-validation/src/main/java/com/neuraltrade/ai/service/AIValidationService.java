// AIValidationService
// Receives TradingSignal from Kafka: signals.raw
// For each signal: build validation prompt → call Claude API → parse response
// Claude Haiku used for speed + cost (< 800ms latency)
// Decision: APPROVE / REJECT / WATCHLIST
// Approved signals published to Kafka: signals.validated
// Rejected signals logged with rejection reason for ML training
// Rate limiting: max 20 concurrent Claude calls
// Fallback: if Claude unavailable → approve signals with score > 80
