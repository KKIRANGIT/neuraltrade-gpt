use crate::CompletedCandle;

pub struct CandlePublisher {
    published: Vec<(String, CompletedCandle)>,
}

impl CandlePublisher {
    pub fn new() -> Self {
        Self { published: Vec::new() }
    }

    pub fn publish(&mut self, candle: CompletedCandle) {
        self.published.push((candle.timeframe.topic().to_string(), candle));
    }

    pub fn published_count(&self) -> usize {
        self.published.len()
    }
}
