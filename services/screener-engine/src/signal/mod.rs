// TradingSignal struct:
//   signal_id, instrument_id, screener_id, signal_direction (+1/-1)
//   entry_zone_low/high, entry_ideal, stop_loss, stop_loss_pct
//   target_1/2/3, target_1/2/3_pct, risk_reward_t1/t2/t3
//   confluence_score, mtf_alignment_score
//   signal_strength: ELITE(>85) / VERY_HIGH(75-85) / HIGH(65-75) / MEDIUM(50-65)
//   conditions_met: u32 bitmask of which conditions passed
//   quantity, capital_required, risk_amount (for 5L account, 1.5% risk)
//   status: ACTIVE / T1_HIT / T2_HIT / T3_HIT / STOPPED / EXPIRED
//
// TradeSetupBuilder:
//   Validates min R:R >= 2.0, stop 0.3%-5%, volume >= 0.8x
//   Calculates position size for configurable account + risk%
//   Returns valid TradingSignal or Err if validation fails
//
// SignalLifecycleManager:
//   Tracks all active signals
//   Updates status on price hitting targets or stop
//   Records outcome for ML training feedback
//   Calculates win rate per screener
