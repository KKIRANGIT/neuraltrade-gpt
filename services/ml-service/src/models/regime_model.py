# RandomForestRegimeClassifier
# Input: Nifty indicators, VIX level+trend, FII flow 5/10/20d,
#        Advance/Decline ratio, sector breadth, global cues
# Output: MarketRegime enum:
#   BULL_TRENDING, BULL_VOLATILE, SIDEWAYS_CALM, SIDEWAYS_CHOPPY,
#   BEAR_VOLATILE, BEAR_TRENDING, CRISIS
# Accuracy target: 78-84%
# Save/load from: models/regime_classifier.pkl
