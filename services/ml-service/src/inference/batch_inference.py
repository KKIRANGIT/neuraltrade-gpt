# BatchInferenceRunner — runs 9:00 AM daily on VPS
# load_all_models(): load pkl and pt files into memory
# run_all_500_stocks(): 
#   fetch features for all 500 → batch numpy array
#   xgb.predict_proba(X) → all 500 probabilities at once
#   lgbm.predict(X) → quality scores
#   lstm batch inference → pattern scores
#   regime.predict() → single market regime
#   anomaly.decision_function(X) → anomaly scores
# store_to_redis(): write all results, TTL 24h
# total target time: < 2 minutes for all 500 stocks
