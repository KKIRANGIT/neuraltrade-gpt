use crate::signal::{build_setup, IndicatorSnapshot, MarketState, TradingSignal};

pub fn evaluate_accumulation(snapshot: &IndicatorSnapshot, market: &MarketState) -> Vec<TradingSignal> {
    if !market.within_market_hours {
        return Vec::new();
    }
    let mut matches = Vec::new();
    if snapshot.volume_ratio >= 2.0 && snapshot.cmf > 0.0 {
        matches.push(signal("SCR13", "Wyckoff", snapshot));
    }
    if snapshot.cmf > 0.05 && snapshot.volume_ratio > 1.0 && snapshot.vwap_side > 0 {
        matches.push(signal("SCR26", "OBV Divergence", snapshot));
    }
    if snapshot.volume_ratio >= 2.5 && snapshot.ema_stack >= 2.0 {
        matches.push(signal("SCR28", "Rounding Bottom", snapshot));
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
