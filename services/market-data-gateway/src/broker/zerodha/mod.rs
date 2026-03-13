use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::models::TickData;

#[derive(Debug, Clone)]
pub struct ZerodhaConnector {
    api_key: String,
    access_token: String,
    connected: bool,
}

impl ZerodhaConnector {
    pub fn connect(api_key: &str, access_token: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            access_token: access_token.to_string(),
            connected: true,
        }
    }

    pub fn subscribe(&self, instrument_tokens: Vec<u32>) -> Vec<TickData> {
        instrument_tokens
            .into_iter()
            .enumerate()
            .map(|(index, instrument_id)| self.mock_tick(instrument_id, index as f32))
            .collect()
    }

    pub fn heartbeat(&self) -> bool {
        self.connected && !self.api_key.is_empty() && !self.access_token.is_empty()
    }

    pub fn reconnect_with_backoff(&mut self) {
        for delay_ms in [100_u64, 300, 900] {
            thread::sleep(Duration::from_millis(delay_ms));
            self.connected = true;
            if self.connected {
                break;
            }
        }
    }

    fn mock_tick(&self, instrument_id: u32, offset: f32) -> TickData {
        let timestamp_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or_default();
        TickData {
            instrument_id,
            ltp: 1000.0 + (instrument_id % 100) as f32 + offset,
            volume: 1_000 + instrument_id as u64,
            oi: 50_000 + instrument_id as u64,
            bid: 999.5 + offset,
            ask: 1000.5 + offset,
            timestamp_ns,
            change_pct: 0.5 + offset * 0.1,
        }
    }
}
