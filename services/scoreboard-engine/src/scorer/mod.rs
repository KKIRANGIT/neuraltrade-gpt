use crate::models::{ScoreInput, TimeframeScore};

pub fn score_timeframe(input: &ScoreInput) -> TimeframeScore {
    let mut bull = 0_u8;
    let mut bear = 0_u8;

    bull += score_condition(input.close > input.ema_9, 5);
    bull += score_condition(input.close > input.ema_20, 5);
    bull += score_condition(input.close > input.ema_50, 7);
    bull += score_condition(input.close > input.ema_200, 8);
    bull += score_condition(input.ema_9 > input.ema_20 && input.ema_20 > input.ema_50, 5);
    bull += score_condition(input.supertrend_direction == 1, 5);
    bull += score_condition((50.0..=70.0).contains(&input.rsi_14), 8);
    bull += score_condition(input.rsi_7 > input.rsi_14, 4);
    bull += score_condition(input.macd_histogram > 0.0, 5);
    bull += score_condition(input.macd_histogram > input.prev_macd_histogram, 4);
    bull += score_condition(input.roc_12 > 2.0, 4);
    bull += score_condition(input.volume_ratio > 1.5, 7);
    bull += score_condition(input.volume_ratio > 2.5, 5);
    bull += score_condition(input.obv_slope_5 > 0.0, 5);
    bull += score_condition(input.cmf_20 > 0.05, 3);
    bull += score_condition(input.vwap_side > 0, 5);
    bull += score_condition(input.bb_squeeze, 4);
    bull += score_condition(input.squeeze_released_up, 6);
    bull += score_condition(input.dc_position_20 > 0.7, 5);
    bull += score_condition(input.adx > 25.0 && input.plus_di > input.minus_di, 5);

    bear += score_condition(input.close < input.ema_9, 5);
    bear += score_condition(input.close < input.ema_20, 5);
    bear += score_condition(input.close < input.ema_50, 7);
    bear += score_condition(input.close < input.ema_200, 8);
    bear += score_condition(input.ema_9 < input.ema_20 && input.ema_20 < input.ema_50, 5);
    bear += score_condition(input.supertrend_direction == -1, 5);
    bear += score_condition((30.0..=50.0).contains(&input.rsi_14), 8);
    bear += score_condition(input.rsi_7 < input.rsi_14, 4);
    bear += score_condition(input.macd_histogram < 0.0, 5);
    bear += score_condition(input.macd_histogram < input.prev_macd_histogram, 4);
    bear += score_condition(input.roc_12 < -2.0, 4);
    bear += score_condition(input.volume_ratio > 1.5, 7);
    bear += score_condition(input.obv_slope_5 < 0.0, 5);
    bear += score_condition(input.cmf_20 < -0.05, 3);
    bear += score_condition(input.vwap_side < 0, 5);
    bear += score_condition(input.dc_position_20 < 0.3, 5);
    bear += score_condition(input.adx > 25.0 && input.plus_di < input.minus_di, 5);

    TimeframeScore {
        timeframe: input.timeframe.clone(),
        bull: bull.min(100),
        bear: bear.min(100),
    }
}

fn score_condition(condition: bool, points: u8) -> u8 {
    if condition {
        points
    } else {
        0
    }
}
