// ValidationPromptBuilder
// buildSignalValidationPrompt(TradingSignal, IndicatorSnapshot, MarketContext) -> String
//   Includes: symbol, direction, entry, stop, target, R:R
//   Includes: all key indicator values (RSI, MACD, volume, supertrend, etc.)
//   Includes: ML scores (XGBoost, LightGBM, LSTM probabilities)
//   Includes: market context (Nifty, VIX, FII, sector, regime)
//   Includes: events risk flag (earnings, budget, RBI)
//   Expected output format: VERDICT / CONFIDENCE / REASON / RED_FLAGS / TELUGU
//
// buildDeepAnalysisPrompt(StockData) -> String
//   Full prompt for on-demand stock deep analysis
//   All chapters: big picture, momentum, volume, etc.
//   Structured output for StockStory assembly
