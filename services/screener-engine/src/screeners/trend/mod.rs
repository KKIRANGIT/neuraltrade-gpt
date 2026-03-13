// SCR01: 52-Week High Breakout
//   close > dc_upper_252, volume_ratio >= 2.0, rsi > 55, ema_stack >= 3
//   cooldown: 24h | valid: 5 days
//
// SCR02: Supertrend Flip + Momentum Confluence
//   supertrend_flipped, rsi > 50, macd_hist > 0, volume > 1.5x
//   cooldown: 4h | valid: 2 days
//
// SCR04: EMA Pullback to Rising EMA
//   price near ema_20, ema_20 rising, rsi 40-55, low volume pullback
//   cooldown: 4h | valid: 2 days
//
// SCR14: HHHL Continuation
//   2 confirmed HH + HL, price at recent HL zone, bounce candle
//   cooldown: 48h | valid: 5 days
//
// SCR15: Golden Cross EMA20xEMA50
//   fresh cross within 3 bars, both EMAs rising, volume >= 2x
//   cooldown: 48h | valid: 5 days
//
// SCR29: ADX Surge from Low (Trend Birth)
//   adx was < 18 five bars ago, now >= 20, +DI > -DI, dc_upper_20 break
//   cooldown: 48h | valid: 5 days
//
// SCR30: High Tight Flag
//   stock doubled in 40 days, tight flag < 25%, breakout on volume >= 2.5x
//   cooldown: 72h | valid: 7 days
