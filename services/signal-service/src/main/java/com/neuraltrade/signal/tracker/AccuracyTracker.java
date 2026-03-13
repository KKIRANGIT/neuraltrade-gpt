// AccuracyTracker
// Tracks win rate per screener, per market regime, per sector
// Records: signal fired, outcome, actual return, hold duration
// Calculates: win_rate, avg_winner, avg_loser, expectancy, sharpe
// Stores in TimescaleDB: signal_outcomes table
// Weekly accuracy report generation
// Feeds data back to ML training pipeline for retraining
// Endpoints: /accuracy/screener/{id}, /accuracy/summary, /accuracy/regime
