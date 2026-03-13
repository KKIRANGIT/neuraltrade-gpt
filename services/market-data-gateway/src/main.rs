use market_data_gateway::broker::fyers::FyersConnector;
use market_data_gateway::broker::zerodha::ZerodhaConnector;
use market_data_gateway::config::load_config;
use market_data_gateway::feed::FeedPublisher;
use market_data_gateway::models::{BrokerType, TickData};
use market_data_gateway::ringbuffer::SpscRingBuffer;

fn main() {
    let config = load_config();
    let instruments: Vec<u32> = config
        .instruments
        .iter()
        .map(|instrument| instrument.instrument_id)
        .collect();

    let ticks = match config.broker_type {
        BrokerType::Zerodha => {
            let connector = ZerodhaConnector::connect(&config.api_key, &config.access_token);
            connector.subscribe(instruments)
        }
        BrokerType::Fyers => {
            let connector = FyersConnector::connect();
            connector.subscribe(instruments)
        }
    };

    let buffer: SpscRingBuffer<TickData, 65_536> = SpscRingBuffer::new();
    for tick in ticks {
        let _ = buffer.push(tick);
    }

    let mut feed = FeedPublisher::new();
    let batch = feed.drain_batch(&buffer, 100);

    println!(
        "market-data-gateway ready on health port {} with {} ticks batched",
        config.health_port,
        batch.len()
    );
}
