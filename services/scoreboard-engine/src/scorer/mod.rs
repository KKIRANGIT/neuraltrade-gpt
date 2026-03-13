// BullScorer: calculate_bull_score(snap: IndicatorSnapshot, tf: Timeframe) -> u8
//   TREND (max 35 pts):
//     close > ema_9: +5, close > ema_20: +5, close > ema_50: +7
//     close > ema_200: +8, ema stack aligned 9>20>50: +5
//     supertrend_direction == +1: +5
//   MOMENTUM (max 25 pts):
//     rsi_14 between 50-70: +8, rsi_7 > rsi_14: +4
//     macd_histogram > 0: +5, macd_histogram rising: +4, roc_12 > 2: +4
//   VOLUME (max 20 pts):
//     volume_ratio > 1.5: +7, volume_ratio > 2.5 bonus: +5
//     obv_slope_5 > 0: +5, cmf_20 > 0.05: +3
//   STRUCTURE (max 20 pts):
//     close > vwap (intraday): +5, bb_squeeze: +4
//     squeeze_released_up: +6, dc_position_20 > 0.7: +5
//     adx > 25 AND plus_di > minus_di: +5
//   clamp result to 0-100
//
// BearScorer: calculate_bear_score(snap, tf) -> u8
//   Mirror logic with inverted conditions:
//     close < ema_9/20/50/200, ema stack bearish order
//     rsi 30-50, macd_histogram < 0 and falling
//     volume on down days, obv_slope < 0
//     close < vwap, squeeze_released_down, dc_position < 0.3
