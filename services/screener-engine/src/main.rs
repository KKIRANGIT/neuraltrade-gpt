use screener_engine::dedup::DedupEngine;
use screener_engine::evaluate_all;
use screener_engine::signal::{IndicatorSnapshot, MarketState};

fn main() {
    let snapshot = IndicatorSnapshot {
        instrument_id: 3045,
        symbol: "RELIANCE".to_string(),
        close: 2978.0,
        volume_ratio: 2.3,
        rsi: 61.0,
        macd_histogram: 1.9,
        ema_stack: 4.0,
        close_above_ema_200: true,
        supertrend_flipped: true,
        supertrend_direction: 1,
        adx: 28.0,
        plus_di: 32.0,
        minus_di: 14.0,
        mtf_alignment_score: 8,
        confluence_score: 86,
        bb_squeeze: false,
        squeeze_released_up: true,
        dc_position_20: 0.82,
        cmf: 0.14,
        vwap_side: 1,
        market_cap_cr: 1_940_000.0,
    };
    let market = MarketState {
        within_market_hours: true,
        regime: "BULL_TRENDING".to_string(),
        sector_positive: true,
        fii_net_buyers_3d: true,
        fii_flow_cr: 742.0,
        expiry_week: false,
    };

    let mut dedup = DedupEngine::new();
    let signals = dedup.filter(evaluate_all(&snapshot, &market));
    println!("screener-engine emitted {} deduplicated signals", signals.len());
}
