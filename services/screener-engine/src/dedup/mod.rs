use std::collections::HashMap;

use crate::signal::TradingSignal;

pub struct DedupEngine {
    seen: HashMap<(String, &'static str), usize>,
}

impl DedupEngine {
    pub fn new() -> Self {
        Self {
            seen: HashMap::new(),
        }
    }

    pub fn filter(&mut self, signals: Vec<TradingSignal>) -> Vec<TradingSignal> {
        signals
            .into_iter()
            .filter(|signal| {
                let key = (signal.symbol.clone(), signal.screener_id);
                let count = self.seen.entry(key).or_insert(0);
                if *count >= 1 {
                    return false;
                }
                *count += 1;
                signal.setup.rr_to_target_2 >= 2.0
            })
            .collect()
    }
}
