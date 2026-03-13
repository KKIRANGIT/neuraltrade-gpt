use crate::signal::{build_setup, IndicatorSnapshot, MarketState, TradingSignal};

pub fn evaluate_trend(snapshot: &IndicatorSnapshot, market: &MarketState) -> Vec<TradingSignal> {
    if !market.within_market_hours || market.regime == "VOLATILE_CRISIS" {
        return Vec::new();
    }
    let mut matches = Vec::new();

    if snapshot.volume_ratio >= 2.0 && snapshot.rsi > 55.0 && snapshot.ema_stack >= 3.0 && snapshot.close_above_ema_200 {
        matches.push(signal("SCR01", "52W Breakout", snapshot));
    }
    if snapshot.supertrend_flipped && snapshot.rsi > 50.0 && snapshot.macd_histogram > 0.0 && snapshot.volume_ratio >= 1.5 {
        matches.push(signal("SCR02", "Supertrend Flip", snapshot));
    }
    if snapshot.ema_stack >= 2.0 && (40.0..=55.0).contains(&snapshot.rsi) && snapshot.volume_ratio < 1.2 {
        matches.push(signal("SCR04", "EMA Pullback", snapshot));
    }
    if snapshot.rsi >= 38.0 && snapshot.rsi <= 55.0 && snapshot.confluence_score >= 70 {
        matches.push(signal("SCR14", "HHHL", snapshot));
    }
    if snapshot.ema_stack >= 3.0 && snapshot.volume_ratio >= 2.0 && snapshot.rsi >= 50.0 && snapshot.rsi <= 72.0 {
        matches.push(signal("SCR15", "Golden Cross", snapshot));
    }
    if snapshot.adx >= 20.0 && snapshot.plus_di > snapshot.minus_di && snapshot.dc_position_20 > 0.8 {
        matches.push(signal("SCR29", "ADX Surge", snapshot));
    }
    if snapshot.volume_ratio >= 2.5 && snapshot.confluence_score >= 80 {
        matches.push(signal("SCR30", "High Tight Flag", snapshot));
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
