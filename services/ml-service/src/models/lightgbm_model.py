# LightGBMSignalQualityScorer
# Input: same features as XGBoost + signal_type (which screener fired)
# Output: signal_quality_score (0.0-1.0)
# Trained on: historical signals + outcomes
# High score = signal historically led to clean moves
# Low score = signal historically led to choppy/reversed moves
# Save/load from: models/lightgbm_quality.pkl
