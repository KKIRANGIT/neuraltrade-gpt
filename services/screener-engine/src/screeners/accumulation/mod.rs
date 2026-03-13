// SCR13: Wyckoff Accumulation Breakout
//   Phase A/B/C/D detection via screener_state
//   Spring detection, AR high breakout, volume >= 2.0x, cmf > 0
//   cooldown: 72h | valid: 7 days
//
// SCR26: Institutional Accumulation (OBV Divergence)
//   OBV up 5%+ while price flat, cmf > 0.05, price starts moving
//   Price trigger: close > ema_20 OR supertrend flip
//   cooldown: 48h | valid: 5 days
//
// SCR28: Rounding Bottom Breakout
//   40+ days of gradual decline then gradual rise (U-shape)
//   Volume mirrors shape: high->low->high
//   Breakout above left rim on volume >= 2.5x
//   cooldown: 72h | valid: 7 days
