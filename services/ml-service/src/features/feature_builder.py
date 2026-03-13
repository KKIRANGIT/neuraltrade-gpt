# FeatureBuilder
# fetch_features_for_stock(instrument_id) -> np.array
#   Pull 750 candles from TimescaleDB
#   Calculate feature vector from raw OHLCV
#   Normalize features (StandardScaler per feature)
#   Handle missing values (forward fill, then 0)
#
# build_sequence_for_lstm(instrument_id) -> np.array shape(60,8)
#   Pull last 60 daily candles
#   Build normalized sequence matrix
#
# build_training_dataset(start_date, end_date) -> (X, y)
#   Walk-forward: no future leakage
#   Label: price rose > 2% in next 5 days
