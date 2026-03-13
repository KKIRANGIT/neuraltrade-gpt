use crate::signal::{build_setup, IndicatorSnapshot, MarketState, TradingSignal};

pub fn evaluate_special(snapshot: &IndicatorSnapshot, market: &MarketState) -> Vec<TradingSignal> {
    let mut matches = Vec::new();
    if snapshot.vwap_side > 0 && snapshot.volume_ratio > 1.5 {
        matches.push(signal("SCR03", "VWAP Reclaim", snapshot));
    }
    if snapshot.rsi > 50.0 && snapshot.confluence_score >= 70 {
        matches.push(signal("SCR06", "RSI Divergence", snapshot));
    }
    if snapshot.volume_ratio >= 4.0 {
        matches.push(signal("SCR07", "Volume Climax", snapshot));
    }
    if snapshot.volume_ratio >= 2.0 && snapshot.vwap_side > 0 {
        matches.push(signal("SCR08", "ORB", snapshot));
    }
    if snapshot.ema_stack >= 4.0 && snapshot.supertrend_direction > 0 && snapshot.volume_ratio > 2.0 {
        matches.push(signal("SCR10", "Perfect Storm", snapshot));
    }
    if snapshot.confluence_score >= 70 {
        matches.push(signal("SCR11", "Demand Zone", snapshot));
    }
    if snapshot.mtf_alignment_score >= 6 {
        matches.push(signal("SCR12", "MTF Surge", snapshot));
    }
    if market.expiry_week && snapshot.confluence_score >= 60 {
        matches.push(signal("SCR23", "Max Pain", snapshot));
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
