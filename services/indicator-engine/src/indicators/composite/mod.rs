use crate::models::{Candle, MarketContext};

#[derive(Debug, Clone, Default)]
pub struct CompositeIndicators {
    pub vwap: f32,
    pub vwap_distance_pct: f32,
    pub vwap_side: i8,
    pub supertrend: f32,
    pub supertrend_direction: i8,
    pub supertrend_flipped: bool,
    pub supertrend_flip_bars_ago: f32,
    pub mtf_alignment_score: u8,
    pub confluence_score: u8,
}

pub struct CompositeInput {
    pub atr_10: f32,
    pub bb_upper: f32,
    pub bb_middle: f32,
    pub ema_20: f32,
    pub ema_stack_score: f32,
    pub rsi_14: f32,
    pub rsi_7: f32,
    pub macd_histogram: f32,
    pub adx: f32,
    pub volume_ratio_20: f32,
    pub close: f32,
    pub close_above_ema_200: bool,
    pub tf_15m_bullish: bool,
    pub tf_1h_bullish: bool,
    pub tf_4h_bullish: bool,
    pub tf_1d_bullish: bool,
    pub ml_probability: f32,
}

pub fn calculate(candles: &[Candle], market: &MarketContext, input: CompositeInput) -> CompositeIndicators {
    let mut cumulative_price_volume = 0.0;
    let mut cumulative_volume = 0.0;
    for candle in candles {
        let typical_price = (candle.high + candle.low + candle.close) / 3.0;
        cumulative_price_volume += typical_price * candle.volume;
        cumulative_volume += candle.volume;
    }
    let vwap = cumulative_price_volume / cumulative_volume.max(0.0001);
    let close = candles.last().map(|candle| candle.close).unwrap_or(0.0);
    let vwap_distance_pct = ((close - vwap) / vwap.max(0.0001)) * 100.0;
    let vwap_side = if close >= vwap { 1 } else { -1 };
    let supertrend = ((candles.last().map(|candle| candle.high).unwrap_or(close)
        + candles.last().map(|candle| candle.low).unwrap_or(close))
        / 2.0)
        - (3.0 * input.atr_10);
    let supertrend_direction = if close > supertrend { 1 } else { -1 };
    let supertrend_flipped = false;
    let tf_points = [input.tf_15m_bullish, input.tf_1h_bullish, input.tf_4h_bullish, input.tf_1d_bullish]
        .iter()
        .filter(|value| **value)
        .count() as u8
        * 2;
    let mut confluence = 0.0;
    if input.ema_stack_score >= 3.0 {
        confluence += 20.0;
    }
    if supertrend_direction > 0 {
        confluence += 10.0;
    }
    if input.close_above_ema_200 {
        confluence += 5.0;
    }
    if (50.0..=70.0).contains(&input.rsi_14) {
        confluence += 10.0;
    }
    if input.rsi_7 > input.rsi_14 {
        confluence += 5.0;
    }
    if input.macd_histogram > 0.0 {
        confluence += 7.0;
    }
    if input.adx > 25.0 {
        confluence += 3.0;
    }
    confluence += match input.volume_ratio_20 {
        value if value > 2.5 => 20.0,
        value if value > 2.0 => 16.0,
        value if value > 1.5 => 12.0,
        value if value > 1.0 => 8.0,
        value if value > 0.8 => 4.0,
        _ => 0.0,
    };
    if vwap_side > 0 {
        confluence += 8.0;
    }
    if close > input.ema_20 {
        confluence += 4.0;
    }
    if input.bb_middle > 0.0 && close > input.bb_middle {
        confluence += 3.0;
    }
    if market.nifty_above_ema20 {
        confluence += 5.0;
    }
    if market.sector_positive {
        confluence += 3.0;
    }
    if market.fii_buying {
        confluence += 2.0;
    }
    if tf_points >= 6 {
        confluence += 10.0;
    }
    if input.ml_probability > 70.0 {
        confluence += 5.0;
    }
    if market.regime == "VOLATILE_CRISIS" {
        confluence = 0.0;
    } else if market.regime == "VOLATILE_SPIKE" {
        confluence *= 0.7;
    }

    CompositeIndicators {
        vwap,
        vwap_distance_pct,
        vwap_side,
        supertrend,
        supertrend_direction,
        supertrend_flipped,
        supertrend_flip_bars_ago: 0.0,
        mtf_alignment_score: tf_points,
        confluence_score: confluence.clamp(0.0, 100.0) as u8,
    }
}
