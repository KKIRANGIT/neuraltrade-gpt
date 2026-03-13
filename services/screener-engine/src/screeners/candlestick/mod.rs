use crate::signal::{build_setup, IndicatorSnapshot, MarketState, TradingSignal};

pub fn evaluate_candlestick(snapshot: &IndicatorSnapshot, market: &MarketState) -> Vec<TradingSignal> {
    if !market.within_market_hours || snapshot.volume_ratio < 1.5 {
        return Vec::new();
    }
    let mut matches = Vec::new();
    if snapshot.rsi >= 25.0 && snapshot.rsi <= 50.0 {
        matches.push(signal("SCR16", "Bullish Engulfing", snapshot));
    }
    if snapshot.rsi >= 30.0 && snapshot.confluence_score >= 65 {
        matches.push(signal("SCR17", "Morning Star", snapshot));
    }
    if snapshot.rsi < 70.0 && snapshot.volume_ratio > 1.6 {
        matches.push(signal("SCR18", "Three White Soldiers", snapshot));
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
