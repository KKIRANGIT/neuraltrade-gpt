use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreInput {
    pub timeframe: String,
    pub close: f32,
    pub ema_9: f32,
    pub ema_20: f32,
    pub ema_50: f32,
    pub ema_200: f32,
    pub supertrend_direction: i8,
    pub rsi_14: f32,
    pub rsi_7: f32,
    pub macd_histogram: f32,
    pub prev_macd_histogram: f32,
    pub roc_12: f32,
    pub volume_ratio: f32,
    pub obv_slope_5: f32,
    pub cmf_20: f32,
    pub vwap_side: i8,
    pub bb_squeeze: bool,
    pub squeeze_released_up: bool,
    pub dc_position_20: f32,
    pub adx: f32,
    pub plus_di: f32,
    pub minus_di: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeframeScore {
    pub timeframe: String,
    pub bull: u8,
    pub bear: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreboardEntry {
    pub instrument_id: u32,
    pub symbol: String,
    pub bull_total: u8,
    pub bear_total: u8,
    pub tier: String,
    pub timeframe_scores: Vec<TimeframeScore>,
}
