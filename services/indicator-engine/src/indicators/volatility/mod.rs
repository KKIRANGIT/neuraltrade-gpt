use crate::indicators::trend::{ema, sma};
use crate::models::Candle;

#[derive(Debug, Clone, Default)]
pub struct VolatilityIndicators {
    pub atr_14: f32,
    pub atr_10: f32,
    pub bb_upper: f32,
    pub bb_middle: f32,
    pub bb_lower: f32,
    pub bb_width: f32,
    pub bb_width_min_20: f32,
    pub dc_upper_20: f32,
    pub dc_lower_20: f32,
    pub dc_upper_252: f32,
    pub dc_lower_252: f32,
    pub kc_upper: f32,
    pub kc_lower: f32,
    pub bb_squeeze: bool,
    pub squeeze_released_up: bool,
    pub squeeze_released_down: bool,
    pub dc_position_20: f32,
}

pub fn calculate(candles: &[Candle]) -> VolatilityIndicators {
    let closes: Vec<f32> = candles.iter().map(|candle| candle.close).collect();
    let atr_14 = atr(candles, 14);
    let atr_10 = atr(candles, 10);
    let bb_middle = sma(&closes, 20);
    let stddev = standard_deviation(&closes[closes.len().saturating_sub(20)..], bb_middle);
    let bb_upper = bb_middle + 2.0 * stddev;
    let bb_lower = bb_middle - 2.0 * stddev;
    let bb_width = if bb_middle != 0.0 {
        ((bb_upper - bb_lower) / bb_middle) * 100.0
    } else {
        0.0
    };
    let bb_width_min_20 = bb_width;
    let dc_upper_20 = candles
        .iter()
        .rev()
        .take(20)
        .map(|candle| candle.high)
        .fold(0.0, f32::max);
    let dc_lower_20 = candles
        .iter()
        .rev()
        .take(20)
        .map(|candle| candle.low)
        .fold(f32::MAX, f32::min);
    let dc_upper_252 = candles.iter().map(|candle| candle.high).fold(0.0, f32::max);
    let dc_lower_252 = candles.iter().map(|candle| candle.low).fold(f32::MAX, f32::min);
    let kc_middle = ema(&closes, 20);
    let kc_upper = kc_middle + (2.0 * atr_10);
    let kc_lower = kc_middle - (2.0 * atr_10);
    let bb_squeeze = (bb_upper - bb_lower) < (kc_upper - kc_lower);
    let close = closes.last().copied().unwrap_or(0.0);
    let squeeze_released_up = !bb_squeeze && close > kc_upper;
    let squeeze_released_down = !bb_squeeze && close < kc_lower;
    let dc_position_20 = if dc_upper_20 > dc_lower_20 {
        (close - dc_lower_20) / (dc_upper_20 - dc_lower_20)
    } else {
        0.0
    };

    VolatilityIndicators {
        atr_14,
        atr_10,
        bb_upper,
        bb_middle,
        bb_lower,
        bb_width,
        bb_width_min_20,
        dc_upper_20,
        dc_lower_20,
        dc_upper_252,
        dc_lower_252,
        kc_upper,
        kc_lower,
        bb_squeeze,
        squeeze_released_up,
        squeeze_released_down,
        dc_position_20,
    }
}

pub fn atr(candles: &[Candle], period: usize) -> f32 {
    let values: Vec<f32> = candles
        .windows(2)
        .map(|window| {
            let prev = &window[0];
            let current = &window[1];
            (current.high - current.low)
                .max((current.high - prev.close).abs())
                .max((current.low - prev.close).abs())
        })
        .collect();
    sma(&values, period)
}

fn standard_deviation(values: &[f32], mean: f32) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let variance =
        values.iter().map(|value| (value - mean).powi(2)).sum::<f32>() / values.len() as f32;
    variance.sqrt()
}
