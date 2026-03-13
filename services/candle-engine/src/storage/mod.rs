use std::collections::{HashMap, VecDeque};

use crate::{Candle, Timeframe};

pub struct CandleStorage {
    buffers: HashMap<(u32, Timeframe), VecDeque<Candle>>,
}

impl CandleStorage {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
        }
    }

    pub fn push(&mut self, candle: Candle) {
        let capacity = match candle.timeframe {
            Timeframe::FourHour => 500,
            _ => 750,
        };
        let buffer = self
            .buffers
            .entry((candle.instrument_id, candle.timeframe))
            .or_insert_with(|| VecDeque::with_capacity(capacity));
        if buffer.len() >= capacity {
            buffer.pop_front();
        }
        buffer.push_back(candle);
    }

    pub fn get_candles(&self, instrument_id: u32, timeframe: Timeframe, count: usize) -> Vec<Candle> {
        self.buffers
            .get(&(instrument_id, timeframe))
            .map(|buffer| buffer.iter().rev().take(count).cloned().collect())
            .unwrap_or_default()
    }
}
