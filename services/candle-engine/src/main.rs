use candle_engine::publisher::CandlePublisher;
use candle_engine::storage::CandleStorage;
use candle_engine::timeframes::MultiTimeframeBuilder;
use candle_engine::TickData;

fn main() {
    let mut builder = MultiTimeframeBuilder::new();
    let mut storage = CandleStorage::new();
    let mut publisher = CandlePublisher::new();

    for minute in 1..=15 {
        let tick = TickData {
            instrument_id: 3045,
            price: 2950.0 + minute as f64,
            volume: 1_000 + minute as u64 * 25,
            minute_of_day: minute,
            timestamp_ns: minute as u64,
        };

        for candle in builder.update_tick(tick) {
            storage.push(candle.clone());
            publisher.publish(candle);
        }
    }

    println!(
        "candle-engine built {} candles for instrument 3045",
        publisher.published_count()
    );
}
