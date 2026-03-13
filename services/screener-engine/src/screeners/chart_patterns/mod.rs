use crate::signal::{build_setup, IndicatorSnapshot, MarketState, TradingSignal};

pub fn evaluate_chart_patterns(snapshot: &IndicatorSnapshot, market: &MarketState) -> Vec<TradingSignal> {
    if !market.within_market_hours {
        return Vec::new();
    }
    let mut matches = Vec::new();
    if snapshot.bb_squeeze && snapshot.squeeze_released_up && snapshot.volume_ratio >= 2.0 && snapshot.rsi > 55.0 {
        matches.push(signal("SCR05", "BB Squeeze", snapshot));
    }
    if snapshot.volume_ratio >= 1.5 && snapshot.confluence_score >= 65 {
        matches.push(signal("SCR09", "Inside Bar", snapshot));
    }
    if snapshot.volume_ratio >= 2.5 && snapshot.confluence_score >= 75 {
        matches.push(signal("SCR19", "Cup & Handle", snapshot));
    }
    if snapshot.volume_ratio >= 2.0 && snapshot.dc_position_20 > 0.75 {
        matches.push(signal("SCR20", "Ascending Triangle", snapshot));
        matches.push(signal("SCR21", "Flag/Pennant", snapshot));
        matches.push(signal("SCR27", "Double Bottom", snapshot));
    }
    matches
}

fn signal(id: &'static str, name: &'static str, snapshot: &IndicatorSnapshot) -> TradingSignal {
    TradingSignal {
        screener_id: id,
        screener_name: name,
        symbol: snapshot.symbol.clone(),
        confluence_score: snapshot.confluence_score,
        setup: build_setup(snapshot.close),
    }
}
