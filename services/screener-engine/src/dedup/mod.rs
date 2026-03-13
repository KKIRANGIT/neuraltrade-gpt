// DedupEngine
// Per-screener cooldown tracking: HashMap<(instrument_id, screener_id), last_signal_time>
// Cross-screener minimum gap: 5 minutes per stock
// Daily cap per stock: maximum 2 signals per trading day
// Sector cap: maximum 4 signals per hour per sector
// Regime blocks: VolatileCrisis blocks ALL signals
// VolatileSpike: only SCR10 (Perfect Storm) allowed
// check_dedup(instrument_id, screener_id) -> bool (true = OK to fire)
// register_signal(instrument_id, screener_id, timestamp) -> void
