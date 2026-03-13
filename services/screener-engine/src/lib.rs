pub mod dedup;
pub mod screeners {
    pub mod accumulation;
    pub mod candlestick;
    pub mod chart_patterns;
    pub mod institutional;
    pub mod special;
    pub mod trend;
}
pub mod signal;

use screeners::{
    accumulation::evaluate_accumulation, candlestick::evaluate_candlestick,
    chart_patterns::evaluate_chart_patterns, institutional::evaluate_institutional,
    special::evaluate_special, trend::evaluate_trend,
};
use signal::{IndicatorSnapshot, MarketState, TradingSignal};

pub fn evaluate_all(snapshot: &IndicatorSnapshot, market: &MarketState) -> Vec<TradingSignal> {
    let mut results = Vec::new();
    results.extend(evaluate_trend(snapshot, market));
    results.extend(evaluate_accumulation(snapshot, market));
    results.extend(evaluate_candlestick(snapshot, market));
    results.extend(evaluate_chart_patterns(snapshot, market));
    results.extend(evaluate_institutional(snapshot, market));
    results.extend(evaluate_special(snapshot, market));
    results
}
