// StockStory entity:
//   instrument_id, symbol, generated_at
//   story_type: RULE_BASED / AI_ENHANCED / ON_DEMAND
//   language: EN / TE
//   chapter_1_big_picture: String
//   chapter_2_momentum: String
//   chapter_3_volume: String
//   chapter_4_price_position: String
//   chapter_5_volatility: String
//   chapter_6_patterns: String
//   chapter_7_market_context: String
//   chapter_8_verdict: String
//   bull_score: int, bear_score: int
//   signal_direction: int (+1/-1/0)
//   trade_setup: TradeSetup (entry, stop, targets)
//   ml_bull_probability: double
//   ml_pattern_score: int
//   watch_for_green: List<String>
//   watch_for_red: List<String>
