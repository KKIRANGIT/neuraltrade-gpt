use indicator_engine::engine::calculate_snapshot;
use indicator_engine::models::{Candle, MarketContext, Timeframe};

fn main() {
    let candles = (0..260)
        .map(|index| Candle {
            instrument_id: 3045,
            timeframe: Timeframe::OneDay,
            timestamp_ns: index as u64,
            open: 2500.0 + index as f32,
            high: 2505.0 + index as f32,
            low: 2494.0 + index as f32,
            close: 2502.0 + index as f32,
            volume: 10_000.0 + index as f32 * 25.0,
        })
        .collect::<Vec<Candle>>();

    let market = MarketContext {
        nifty_above_ema20: true,
        fii_buying: true,
        sector_positive: true,
        regime: "BULL_TRENDING".to_string(),
    };

    let snapshot = calculate_snapshot(&candles, &market, 74.0, (true, true, true, true));
    println!(
        "indicator-engine calculated snapshot for instrument {} with confluence {}",
        snapshot.instrument_id, snapshot.confluence_score
    );
}
