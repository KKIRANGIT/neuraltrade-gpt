use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::models::TickData;

#[derive(Debug, Clone)]
pub struct FyersConnector;

impl FyersConnector {
    pub fn connect() -> Self {
        Self
    }

    pub fn subscribe(&self, instrument_tokens: Vec<u32>) -> Vec<TickData> {
        instrument_tokens
            .into_iter()
            .enumerate()
            .map(|(index, instrument_id)| TickData {
                instrument_id,
                ltp: 950.0 + index as f32 * 2.5,
                volume: 8_500 + index as u64 * 50,
                oi: 25_000 + index as u64 * 100,
                bid: 949.5 + index as f32 * 2.5,
                ask: 950.5 + index as f32 * 2.5,
                timestamp_ns: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(0))
                    .as_nanos() as u64,
                change_pct: 0.3 + index as f32 * 0.08,
            })
            .collect()
    }

    pub fn fetch_historical(
        &self,
        symbol: &str,
        timeframe: &str,
        from: &str,
        to: &str,
    ) -> String {
        format!(
            "historical load queued: symbol={symbol}, timeframe={timeframe}, from={from}, to={to}"
        )
    }
}
