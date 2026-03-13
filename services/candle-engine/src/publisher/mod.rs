// CandlePublisher
// On candle close event: publish to Kafka topic candles.{timeframe}
// Message: CompletedCandle { instrument_id, tf, ohlcv, timestamp }
// Also updates Redis key: candle:{instrument_id}:{tf}:latest
