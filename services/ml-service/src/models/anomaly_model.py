# IsolationForestAnomalyDetector
# Detects unusual patterns per stock:
#   Volume spikes without price movement (possible accumulation)
#   Price spikes without volume (operator pump)
#   Pre-announcement unusual activity
# Input: last 30 days of [volume_ratio, price_change, obv_change, cmf]
# Output: anomaly_score (0.0-1.0), anomaly_type: VOLUME/PRICE/DIVERGENCE
# Save/load from: models/anomaly_detector.pkl
