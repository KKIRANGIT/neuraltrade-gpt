use rayon::join;

use crate::indicators::composite::{calculate as calculate_composite, CompositeInput};
use crate::indicators::momentum::calculate as calculate_momentum;
use crate::indicators::trend::calculate as calculate_trend;
use crate::indicators::volatility::calculate as calculate_volatility;
use crate::indicators::volume::calculate as calculate_volume;
use crate::models::{Candle, IndicatorSnapshot, MarketContext, Timeframe};

pub fn calculate_snapshot(
    candles: &[Candle],
    market: &MarketContext,
    ml_probability: f32,
    mtf_flags: (bool, bool, bool, bool),
) -> IndicatorSnapshot {
    let ((trend, momentum), (volume, volatility)) = join(
        || join(|| calculate_trend(candles), || calculate_momentum(candles)),
        || join(|| calculate_volume(candles), || calculate_volatility(candles)),
    );
    let close = candles.last().map(|candle| candle.close).unwrap_or(0.0);
    let composite = calculate_composite(
        candles,
        market,
        CompositeInput {
            atr_10: volatility.atr_10,
            bb_upper: volatility.bb_upper,
            bb_middle: volatility.bb_middle,
            ema_20: trend.ema_20,
            ema_stack_score: trend.ema_stack_score,
            rsi_14: trend.rsi_14,
            rsi_7: trend.rsi_7,
            macd_histogram: momentum.macd_histogram,
            adx: trend.adx,
            volume_ratio_20: volume.volume_ratio_20,
            close,
            close_above_ema_200: close > trend.ema_200,
            tf_15m_bullish: mtf_flags.0,
            tf_1h_bullish: mtf_flags.1,
            tf_4h_bullish: mtf_flags.2,
            tf_1d_bullish: mtf_flags.3,
            ml_probability,
        },
    );

    IndicatorSnapshot {
        instrument_id: candles.last().map(|candle| candle.instrument_id).unwrap_or(0),
        timeframe: encode_timeframe(candles.last().map(|candle| candle.timeframe).unwrap_or(Timeframe::OneDay)),
        timestamp_ns: candles.last().map(|candle| candle.timestamp_ns).unwrap_or(0),
        bars_seen: candles.len() as u16,
        ema_9: trend.ema_9,
        ema_20: trend.ema_20,
        ema_50: trend.ema_50,
        ema_200: trend.ema_200,
        sma_20: trend.sma_20,
        sma_50: trend.sma_50,
        rsi_14: trend.rsi_14,
        rsi_7: trend.rsi_7,
        macd: momentum.macd,
        macd_signal: momentum.macd_signal,
        macd_histogram: momentum.macd_histogram,
        atr_14: volatility.atr_14,
        atr_10: volatility.atr_10,
        bb_upper: volatility.bb_upper,
        bb_middle: volatility.bb_middle,
        bb_lower: volatility.bb_lower,
        bb_width: volatility.bb_width,
        bb_width_min_20: volatility.bb_width_min_20,
        obv: volume.obv,
        obv_sma_20: volume.obv_sma_20,
        obv_slope_5: volume.obv_slope_5,
        volume_ratio_20: volume.volume_ratio_20,
        roc_12: momentum.roc_12,
        stochastic_k: momentum.stochastic_k,
        williams_r: momentum.williams_r,
        dc_upper_20: volatility.dc_upper_20,
        dc_lower_20: volatility.dc_lower_20,
        dc_upper_252: volatility.dc_upper_252,
        dc_lower_252: volatility.dc_lower_252,
        distance_to_52w_high: ((volatility.dc_upper_252 - close) / volatility.dc_upper_252.max(0.0001)) * 100.0,
        distance_from_52w_low: ((close - volatility.dc_lower_252) / close.max(0.0001)) * 100.0,
        ema_stack_score: trend.ema_stack_score,
        consecutive_green: trend.consecutive_green,
        consecutive_red: trend.consecutive_red,
        supertrend: composite.supertrend,
        supertrend_flip_bars_ago: composite.supertrend_flip_bars_ago,
        adx: trend.adx,
        plus_di: trend.plus_di,
        minus_di: trend.minus_di,
        vwap: composite.vwap,
        vwap_distance_pct: composite.vwap_distance_pct,
        cmf_20: volume.cmf_20,
        force_index_2: momentum.force_index_2,
        stochastic_d: momentum.stochastic_d,
        kc_upper: volatility.kc_upper,
        kc_lower: volatility.kc_lower,
        dc_position_20: volatility.dc_position_20,
        mtf_alignment_score: composite.mtf_alignment_score,
        confluence_score: composite.confluence_score,
        ema_200_ready: trend.ema_200_ready,
        adx_ready: trend.adx_ready,
        bb_squeeze: volatility.bb_squeeze,
        supertrend_direction: composite.supertrend_direction,
        supertrend_flipped: composite.supertrend_flipped,
        vwap_side: composite.vwap_side,
        squeeze_released_up: volatility.squeeze_released_up,
        squeeze_released_down: volatility.squeeze_released_down,
        tf_15m_bullish: mtf_flags.0,
        tf_1h_bullish: mtf_flags.1,
        tf_4h_bullish: mtf_flags.2,
        tf_1d_bullish: mtf_flags.3,
    }
}

fn encode_timeframe(timeframe: Timeframe) -> u8 {
    match timeframe {
        Timeframe::OneMinute => 1,
        Timeframe::FiveMinute => 5,
        Timeframe::FifteenMinute => 15,
        Timeframe::OneHour => 60,
        Timeframe::FourHour => 240,
        Timeframe::OneDay => 255,
    }
}
