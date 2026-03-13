# LSTMPatternMatcher
# Input sequence: last 60 days of [close, volume, rsi, macd, ema_ratio, bb_pos, adx, vwap_dist]
# Shape: (batch, 60, 8)
# Output: pattern_similarity_score (0-100), next_5d_return_prediction (float)
# Architecture: 2-layer LSTM → Dense → Output
# Training: PyTorch with MPS backend (MacBook M2)
# Save/load from: models/lstm_pattern.pt
# Groups stocks into sectors for shared training
