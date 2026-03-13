use crate::indicators::trend::{ema, sma};
use crate::models::Candle;

#[derive(Debug, Clone, Default)]
pub struct MomentumIndicators {
    pub macd: f32,
    pub macd_signal: f32,
    pub macd_histogram: f32,
    pub roc_12: f32,
    pub stochastic_k: f32,
    pub stochastic_d: f32,
    pub williams_r: f32,
    pub force_index_2: f32,
}

pub fn calculate(candles: &[Candle]) -> MomentumIndicators {
    let closes: Vec<f32> = candles.iter().map(|candle| candle.close).collect();
    let highs: Vec<f32> = candles.iter().map(|candle| candle.high).collect();
    let lows: Vec<f32> = candles.iter().map(|candle| candle.low).collect();
    let macd = ema(&closes, 12) - ema(&closes, 26);
    let macd_series = closes
        .iter()
        .enumerate()
        .map(|(index, _)| ema(&closes[..=index], 12) - ema(&closes[..=index], 26))
        .collect::<Vec<f32>>();
    let macd_signal = ema(&macd_series, 9);
    let macd_histogram = macd - macd_signal;
    let roc_12 = if closes.len() > 12 {
        let reference = closes[closes.len() - 13];
        ((closes.last().copied().unwrap_or(0.0) - reference) / reference.max(0.0001)) * 100.0
    } else {
        0.0
    };
    let period_high = highs.iter().rev().take(14).copied().fold(0.0, f32::max);
    let period_low = lows
        .iter()
        .rev()
        .take(14)
        .copied()
        .fold(f32::MAX, f32::min);
    let close = closes.last().copied().unwrap_or(0.0);
    let stochastic_k = if period_high > period_low {
        ((close - period_low) / (period_high - period_low)) * 100.0
    } else {
        0.0
    };
    let stochastic_d = sma(&[stochastic_k, stochastic_k, stochastic_k], 3);
    let williams_r = if period_high > period_low {
        ((period_high - close) / (period_high - period_low)) * -100.0
    } else {
        0.0
    };
    let force_index_values: Vec<f32> = candles
        .windows(2)
        .map(|window| (window[1].close - window[0].close) * window[1].volume)
        .collect();
    let force_index_2 = ema(&force_index_values, 2);

    MomentumIndicators {
        macd,
        macd_signal,
        macd_histogram,
        roc_12,
        stochastic_k,
        stochastic_d,
        williams_r,
        force_index_2,
    }
}
