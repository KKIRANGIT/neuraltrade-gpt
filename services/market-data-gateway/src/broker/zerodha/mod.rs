// ZerodhaConnector
// Connect to Zerodha Kite WebSocket API.
// Subscribe to 500 instrument tokens.
// Receive tick structs: instrument_id, ltp, volume, timestamp_ns.
// On tick received: push to SPSC ring buffer.
// Handle reconnection on disconnect automatically.
