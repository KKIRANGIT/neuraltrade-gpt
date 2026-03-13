# NEURALTRADE — ML Service
# Batch inference: runs every day at 9:00 AM before market opens.
# Loads all trained models (XGBoost, LightGBM, LSTM, RandomForest).
# Fetches latest IndicatorSnapshots for all 500 stocks from Redis.
# Runs inference on all models for all stocks in batch.
# Stores ML scores in Redis: ml:{instrument_id}:scores
# Also exposes HTTP endpoint for on-demand inference.
# Retraining: triggered every Sunday at 9 PM.
