pub mod publisher;
pub mod storage;
pub mod timeframes;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Timeframe {
    OneMinute,
    FiveMinute,
    FifteenMinute,
    OneHour,
    FourHour,
    OneDay,
}

impl Timeframe {
    pub fn minutes(self) -> u32 {
        match self {
            Timeframe::OneMinute => 1,
            Timeframe::FiveMinute => 5,
            Timeframe::FifteenMinute => 15,
            Timeframe::OneHour => 60,
            Timeframe::FourHour => 240,
            Timeframe::OneDay => 375,
        }
    }

    pub fn topic(self) -> &'static str {
        match self {
            Timeframe::OneMinute => "candles.1m",
            Timeframe::FiveMinute => "candles.5m",
            Timeframe::FifteenMinute => "candles.15m",
            Timeframe::OneHour => "candles.1h",
            Timeframe::FourHour => "candles.4h",
            Timeframe::OneDay => "candles.1d",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickData {
    pub instrument_id: u32,
    pub price: f64,
    pub volume: u64,
    pub minute_of_day: u32,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub instrument_id: u32,
    pub timeframe: Timeframe,
    pub open_time: u32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

pub type CompletedCandle = Candle;
