use std::collections::HashMap;

use crate::models::TickData;
use crate::ringbuffer::SpscRingBuffer;

pub struct FeedPublisher {
    latest_ticks: HashMap<u32, TickData>,
}

impl FeedPublisher {
    pub fn new() -> Self {
        Self {
            latest_ticks: HashMap::new(),
        }
    }

    pub fn drain_batch<const N: usize>(
        &mut self,
        buffer: &SpscRingBuffer<TickData, N>,
        batch_size: usize,
    ) -> Vec<TickData> {
        let mut drained = Vec::with_capacity(batch_size);
        while drained.len() < batch_size {
            match buffer.pop() {
                Some(tick) => {
                    self.latest_ticks.insert(tick.instrument_id, tick.clone());
                    drained.push(tick);
                }
                None => break,
            }
        }
        drained
    }

    pub fn get_latest_tick(&self, instrument_id: u32) -> Option<&TickData> {
        self.latest_ticks.get(&instrument_id)
    }
}
