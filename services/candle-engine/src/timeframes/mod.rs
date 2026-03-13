// TimeframeManager
// Maintains active (open) candle state per instrument per timeframe.
// CandleState: open, high, low, close, volume, open_time, close_time.
// On each tick:
//   Update all 6 timeframe candles simultaneously.
//   Check if any timeframe candle closed (boundary check).
//   If closed: emit CompletedCandle event.
// Pre-computed boundary table for fast candle close detection.
// Handles 9:15 AM daily reset for intraday timeframes.
