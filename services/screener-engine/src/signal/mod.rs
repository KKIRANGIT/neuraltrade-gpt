use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorSnapshot {
    pub instrument_id: u32,
    pub symbol: String,
    pub close: f32,
    pub volume_ratio: f32,
    pub rsi: f32,
    pub macd_histogram: f32,
    pub ema_stack: f32,
    pub close_above_ema_200: bool,
    pub supertrend_flipped: bool,
    pub supertrend_direction: i8,
    pub adx: f32,
    pub plus_di: f32,
    pub minus_di: f32,
    pub mtf_alignment_score: u8,
    pub confluence_score: u8,
    pub bb_squeeze: bool,
    pub squeeze_released_up: bool,
    pub dc_position_20: f32,
    pub cmf: f32,
    pub vwap_side: i8,
    pub market_cap_cr: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketState {
    pub within_market_hours: bool,
    pub regime: String,
    pub sector_positive: bool,
    pub fii_net_buyers_3d: bool,
    pub fii_flow_cr: f32,
    pub expiry_week: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeSetup {
    pub entry_ideal: f32,
    pub entry_zone_low: f32,
    pub entry_zone_high: f32,
    pub stop_loss: f32,
    pub target_1: f32,
    pub target_2: f32,
    pub target_3: f32,
    pub rr_to_target_2: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSignal {
    pub screener_id: &'static str,
    pub screener_name: &'static str,
    pub symbol: String,
    pub confluence_score: u8,
    pub setup: TradeSetup,
}

pub fn build_setup(close: f32) -> TradeSetup {
    let entry = close;
    let stop = close * 0.975;
    let target_1 = close * 1.02;
    let target_2 = close * 1.04;
    let target_3 = close * 1.06;
    let risk = (entry - stop).max(0.01);
    let reward = target_2 - entry;
    TradeSetup {
        entry_ideal: entry,
        entry_zone_low: close * 0.995,
        entry_zone_high: close * 1.003,
        stop_loss: stop,
        target_1,
        target_2,
        target_3,
        rr_to_target_2: reward / risk,
    }
}
