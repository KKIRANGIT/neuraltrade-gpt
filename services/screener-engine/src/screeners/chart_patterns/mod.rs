// SCR05: Bollinger Band Squeeze Breakout
//   bb_squeeze duration >= 5 bars, squeeze_released_up = true
//   volume >= 2.0x, rsi > 55
//   cooldown: 48h | valid: 3 days
//
// SCR09: Inside Bar Breakout
//   Current candle high/low inside prior candle range
//   Breakout direction confirmed by volume
//   cooldown: 48h | valid: 2 days
//
// SCR19: Cup and Handle Breakout
//   Cup: 20-60 days, 12-35% depth, U-shape
//   Handle: 5-20 days, < 15% pullback, declining volume
//   Breakout above handle_high on volume >= 2.5x
//   cooldown: 72h | valid: 7 days
//
// SCR20: Ascending Triangle Breakout
//   Flat resistance 2+ touches, rising support (higher lows)
//   Breakout above resistance on volume >= 2.0x
//   cooldown: 48h | valid: 5 days
//
// SCR21: Flag and Pennant Continuation
//   Pole: 5%+ move in 3-5 days, flag: 5-15 days tight consolidation
//   Breakout above flag upper trendline on volume >= 2.0x
//   cooldown: 48h | valid: 5 days
//
// SCR27: Double Bottom Breakout
//   Two lows within 2% of each other, volume at 2nd bottom < 1st
//   RSI divergence at 2nd bottom, neckline breakout on volume >= 2.0x
//   cooldown: 48h | valid: 5 days
