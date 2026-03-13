// Signal JPA entity mapped to TimescaleDB signals table
// All fields from TradingSignal struct (Rust → Java DTO)
// Additional fields: validated_at, claude_verdict, claude_reason
// Relationship: belongs to instrument, belongs to screener
// Indexes: instrument_id+status, screener_id+created_at
