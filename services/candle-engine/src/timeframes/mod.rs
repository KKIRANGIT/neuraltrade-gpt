use std::collections::HashMap;

use crate::{CompletedCandle, TickData, Timeframe};

#[derive(Debug, Clone)]
pub struct TimeframeState {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub open_time: u32,
}

impl TimeframeState {
    fn from_tick(tick: &TickData) -> Self {
        Self {
            open: tick.price,
            high: tick.price,
            low: tick.price,
            close: tick.price,
            volume: tick.volume,
            open_time: tick.minute_of_day,
        }
    }

    fn update(&mut self, tick: &TickData) {
        self.high = self.high.max(tick.price);
        self.low = self.low.min(tick.price);
        self.close = tick.price;
        self.volume += tick.volume;
    }
}

pub struct MultiTimeframeBuilder {
    states: HashMap<(u32, Timeframe), TimeframeState>,
}

impl MultiTimeframeBuilder {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    pub fn update_tick(&mut self, tick: TickData) -> Vec<CompletedCandle> {
        let mut completed = Vec::new();
        for timeframe in [
            Timeframe::OneMinute,
            Timeframe::FiveMinute,
            Timeframe::FifteenMinute,
            Timeframe::OneHour,
            Timeframe::FourHour,
            Timeframe::OneDay,
        ] {
            let key = (tick.instrument_id, timeframe);
            let state = self
                .states
                .entry(key)
                .or_insert_with(|| TimeframeState::from_tick(&tick));
            state.update(&tick);
            if closes_on_minute(tick.minute_of_day, timeframe) {
                completed.push(CompletedCandle {
                    instrument_id: tick.instrument_id,
                    timeframe,
                    open_time: state.open_time,
                    open: state.open,
                    high: state.high,
                    low: state.low,
                    close: state.close,
                    volume: state.volume,
                });
                *state = TimeframeState::from_tick(&tick);
            }
        }
        completed
    }

    pub fn reset_for_open(&mut self) {
        self.states.clear();
    }
}

pub fn closes_on_minute(minute_of_day: u32, timeframe: Timeframe) -> bool {
    match timeframe {
        Timeframe::OneMinute => minute_of_day >= 1,
        Timeframe::FiveMinute => minute_of_day % 5 == 0,
        Timeframe::FifteenMinute => minute_of_day % 15 == 0,
        Timeframe::OneHour => minute_of_day % 60 == 0,
        Timeframe::FourHour => minute_of_day == 240,
        Timeframe::OneDay => minute_of_day == 375,
    }
}
