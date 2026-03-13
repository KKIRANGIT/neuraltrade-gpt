// IndicatorOrchestrator
// Receives candle close event.
// Fetches candle history from CandleStorage (750 candles).
// Runs Wave1 indicators (no dependencies) in parallel via Rayon.
// Runs Wave2 indicators (depend on Wave1) sequentially after Wave1.
// Runs Wave3 composite scores last.
// Assembles IndicatorSnapshot struct (256-byte cache-aligned).
// Publishes snapshot to Kafka + Redis.
// Target: all 500 stocks × all indicators in < 200ms.
