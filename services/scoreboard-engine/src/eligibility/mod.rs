// EligibilityGate
// BULL eligible: bull_1d >= 50 AND bull_1h >= 45 AND bull_total >= 60
//   AND bull_total > bear_total + 15 AND volume_ratio >= 1.0
//   AND regime != VolatileCrisis AND close > ema_200_1d
// BEAR eligible: bear_1d >= 50 AND bear_1h >= 45 AND bear_total >= 60
//   AND bear_total > bull_total + 15 AND volume_ratio >= 1.0
// Signal fire thresholds:
//   >= 80: ELITE signal → fire immediately
//   70-79: HIGH signal → fire after dedup check
//   60-69: MEDIUM → add to watchlist only, no signal
// Returns: EligibilityResult { bull_eligible, bear_eligible, signal_strength }
