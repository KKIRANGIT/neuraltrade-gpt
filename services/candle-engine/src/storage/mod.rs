// CandleStorage
// Writes completed candles to TimescaleDB table: candles
// Maintains in-memory ring buffer of last N candles per instrument per TF:
//   1M:  750 candles, 5M: 750, 15M: 750, 1H: 750, 4H: 500, 1D: 750
// Provides: get_candles(instrument_id, timeframe, count) -> Vec<Candle>
// Used by indicator engine to fetch candle history.
