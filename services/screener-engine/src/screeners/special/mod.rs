// SCR03: VWAP Reclaim with Momentum
//   Price crosses above VWAP after being below, volume > 1.5x
//   Only active 9:30 AM - 1:30 PM, intraday only
//   cooldown: 75min | valid: same day
//
// SCR06: RSI Bullish Divergence
//   Price making lower lows, RSI making higher lows (bullish divergence)
//   Minimum 2 swing points for divergence, close > prior swing high trigger
//   cooldown: 48h | valid: 5 days
//
// SCR07: Volume Climax Reversal
//   volume_ratio >= 4.0, price at support zone
//   Long lower wick (hammer-like), close > open
//   cooldown: 48h | valid: 3 days
//
// SCR08: Opening Range Breakout (ORB)
//   orb_high/orb_low set from 9:15-9:30 AM first 15 minutes
//   Break above orb_high on volume >= 2.0x between 9:30-11:00 AM
//   cooldown: 24h | valid: same day until 1:30 PM
//
// SCR10: Perfect Storm (Multi-Factor)
//   6+ conditions from different categories all bullish simultaneously
//   ema_stack=4, supertrend=bull, rsi>55, macd>0, volume>2x, vwap_above
//   cooldown: 24h | valid: 3 days
//
// SCR11: Demand Zone Bounce
//   Price returns to identified demand zone (prior accumulation area)
//   Bounce candle with volume, rsi < 50, confluence >= 70
//   cooldown: 48h | valid: 5 days
//
// SCR12: Multi-Timeframe Momentum Surge
//   mtf_alignment_score >= 6 (6 of 8 timeframe checks bullish)
//   All: 15M 1H 4H 1D all showing bullish indicators
//   cooldown: 3h | valid: 2 days
//
// SCR23: Max Pain Convergence (Expiry Week Only)
//   Only active when is_expiry_week = true
//   Price away from max_pain_level >= 1.5%
//   Signal direction: toward max pain level
//   cooldown: 24h | valid: expiry day only
