# XGBoostDirectionPredictor
# Input features (per stock per day):
#   All 39 indicator values (float32)
#   Last 10 days of: rsi, macd_hist, volume_ratio, ema_stack, supertrend
#   Sector performance, FII net flow, India VIX
#   Day of week, days to expiry
# Output: bull_probability (0.0-1.0), bear_probability (0.0-1.0)
# Training data: 3 years × 500 stocks = 375,000 samples
# Label: did price rise > 2% within 5 days? yes=1 no=0
# Save/load from: models/xgboost_direction.pkl
