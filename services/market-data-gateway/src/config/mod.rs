use crate::models::{BrokerType, GatewayConfig, InstrumentMeta};

pub fn load_config() -> GatewayConfig {
    GatewayConfig {
        broker_type: BrokerType::Zerodha,
        api_key: std::env::var("ZERODHA_API_KEY").unwrap_or_else(|_| "demo-key".to_string()),
        access_token: std::env::var("ZERODHA_ACCESS_TOKEN")
            .unwrap_or_else(|_| "demo-token".to_string()),
        health_port: 9001,
        instruments: vec![
            InstrumentMeta {
                instrument_id: 3045,
                symbol: "RELIANCE".to_string(),
                exchange: "NSE".to_string(),
                lot_size: 1,
                tick_size: 0.05,
                sector: "Energy".to_string(),
            },
            InstrumentMeta {
                instrument_id: 1594,
                symbol: "TCS".to_string(),
                exchange: "NSE".to_string(),
                lot_size: 1,
                tick_size: 0.05,
                sector: "IT".to_string(),
            },
            InstrumentMeta {
                instrument_id: 4963,
                symbol: "HDFCBANK".to_string(),
                exchange: "NSE".to_string(),
                lot_size: 1,
                tick_size: 0.05,
                sector: "Banking".to_string(),
            },
        ],
    }
}
