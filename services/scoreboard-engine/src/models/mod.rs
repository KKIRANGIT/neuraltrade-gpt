// StockScoreboard struct:
//   instrument_id: u32, symbol: [u8;16]
//   bull_1m/5m/15m/1h/1d: u8 (0-100 each)
//   bear_1m/5m/15m/1h/1d: u8
//   bull_total: u8, bear_total: u8
//   bull_conditions_1m/.../1d: u16 (bitmask of passed conditions)
//   bear_conditions_1m/.../1d: u16
//   signal_direction: i8 (+1 BULL / -1 BEAR / 0 NEUTRAL)
//   signal_strength: SignalStrength enum
//   confluence_score: u8
//   timestamp_ns: u64
// ScoreboardSnapshot: Vec<StockScoreboard> for all 500 stocks
