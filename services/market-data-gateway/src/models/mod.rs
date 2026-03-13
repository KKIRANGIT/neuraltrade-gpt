use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickData {
    pub instrument_id: u32,
    pub ltp: f32,
    pub volume: u64,
    pub oi: u64,
    pub bid: f32,
    pub ask: f32,
    pub timestamp_ns: u64,
    pub change_pct: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentMeta {
    pub instrument_id: u32,
    pub symbol: String,
    pub exchange: String,
    pub lot_size: u32,
    pub tick_size: f32,
    pub sector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrokerType {
    Zerodha,
    Fyers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    pub broker_type: BrokerType,
    pub api_key: String,
    pub access_token: String,
    pub health_port: u16,
    pub instruments: Vec<InstrumentMeta>,
}
