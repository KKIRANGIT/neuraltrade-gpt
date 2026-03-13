use crate::signal::{build_setup, IndicatorSnapshot, MarketState, TradingSignal};

pub fn evaluate_institutional(snapshot: &IndicatorSnapshot, market: &MarketState) -> Vec<TradingSignal> {
    let mut matches = Vec::new();
    if market.fii_net_buyers_3d && market.fii_flow_cr >= 500.0 && market.sector_positive && snapshot.market_cap_cr > 5_000.0 {
        matches.push(signal("SCR22", "FII Confluence", snapshot));
    }
    if market.sector_positive && snapshot.ema_stack >= 2.0 && snapshot.volume_ratio >= 1.5 {
        matches.push(signal("SCR24", "Sector Rotation", snapshot));
    }
    if snapshot.volume_ratio >= 1.8 && snapshot.confluence_score >= 70 {
        matches.push(signal("SCR25", "Earnings Gap", snapshot));
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
