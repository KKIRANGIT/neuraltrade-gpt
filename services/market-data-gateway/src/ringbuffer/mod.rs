// SPSC Ring Buffer (Single Producer Single Consumer)
// Lock-free circular buffer for tick data.
// Producer: broker websocket thread.
// Consumer: candle engine thread.
// Size: 65536 slots (power of 2).
// Each slot: TickData struct (instrument_id, price, volume, ts_ns).
