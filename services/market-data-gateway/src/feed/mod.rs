// FeedPublisher
// Reads from SPSC ring buffer.
// Publishes ticks to Kafka topic: raw.ticks
// Also maintains in-memory latest tick per instrument.
// Provides get_latest_tick(instrument_id) -> TickData
