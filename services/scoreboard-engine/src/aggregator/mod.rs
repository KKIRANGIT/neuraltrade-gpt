// ScoreboardAggregator
// Collects bull/bear scores for all 5 timeframes.
// Applies weights: 1M*0.05 + 5M*0.10 + 15M*0.20 + 1H*0.30 + 1D*0.35
// Calculates MTF alignment bonus:
//   Count timeframes where bull_tf >= 50
//   5/5 = +15, 4/5 = +10, 3/5 = +5 (same for bear)
// Caps total at 100.
// Assembles full StockScoreboard struct.
// Stores bull_conditions bitmasks per timeframe for UI display.
