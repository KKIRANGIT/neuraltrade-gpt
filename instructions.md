# NeuralTrade — Master Requirements Document
# Read this file completely before writing any code.
# This is the single source of truth for the entire system.

---

## WHAT THIS SYSTEM IS

NeuralTrade is a stock market analysis and signal generation platform
for Indian equity markets (NSE/BSE). It analyses 500 stocks across
5 timeframes using 39 technical indicators, 30 screeners, ML models,
and Claude AI to generate buy/sell signals with Telugu and English
story-based explanations. Target: 80% analytical accuracy.

Target User: Indian retail traders who want professional-grade
analysis explained in simple Telugu/English without needing to
understand charts or indicators themselves.

Target Revenue: ₹2 lakh/month from 200+ paying subscribers.

---

## HARDWARE AND HOSTING

MacBook M2 Pro (16GB, 12 cores, MPS GPU):
  - Weekly ML model training (every Sunday 9 PM)
  - Local development environment

Hostinger VPS Mumbai (16GB RAM, Ubuntu 24.04):
  - All production services run here
  - Close to NSE/BSE for low latency
  - Single server (no Kubernetes needed at this stage)

---

## TECH STACK — NON-NEGOTIABLE

| Purpose | Technology |
|---------|-----------|
| Hot path (tick→candle→indicator→screener) | Rust 1.75+ |
| Platform services (API, stories, validation) | Java 21 Spring Boot 3.2 |
| ML training and inference | Python 3.11 |
| Frontend | Next.js 14 |
| Primary database | TimescaleDB (PostgreSQL 16) |
| Cache | Redis 7.2 |
| Message bus | Kafka 3.6 KRaft |
| AI | Claude API (Haiku for bulk, Sonnet for deep) |
| Monitoring | Prometheus + Grafana |

---

## SYSTEM ARCHITECTURE — HOW DATA FLOWS

```
MARKET DATA (NSE/BSE)
       ↓
[1] Market Data Gateway (Rust)
    Zerodha/Fyers WebSocket → ticks
    SPSC ring buffer → Kafka topic: raw.ticks
       ↓
[2] Candle Engine (Rust)
    Consumes raw.ticks
    Builds candles: 1M, 5M, 15M, 1H, 4H, 1D simultaneously
    Stores in TimescaleDB + in-memory ring buffer
    Publishes to Kafka: candles.{timeframe}
       ↓
[3] Indicator Engine (Rust + Rayon)
    Consumes candles.{timeframe}
    Calculates all 39 indicators in parallel
    Outputs IndicatorSnapshot per stock per TF
    Publishes to Kafka: indicators.snapshot
    Caches in Redis: indicator:{id}:{tf}
       ↓
    ┌──────────────────────────────────────┐
    │                                      │
[4] Screener Engine (Rust)         [5] Scoreboard Engine (Rust)
    30 quantum screeners               Bull/Bear scoring per TF
    DedupEngine cooldowns              Weighted aggregation
    TradeSetup builder                 Eligibility gates
    Kafka: signals.raw                 Kafka: signals.raw
    │                                      │
    └──────────────────────────────────────┘
                      ↓
[6] ML Service (Python)
    Runs at 9:00 AM daily (batch)
    XGBoost + LightGBM + LSTM + RandomForest
    Scores all 500 stocks
    Redis: ml:{instrument_id}:scores
                      ↓
[7] AI Validation Service (Java + Claude API)
    Consumes signals.raw
    Sends each signal to Claude Haiku for validation
    APPROVE / REJECT / WATCHLIST decision
    Kafka: signals.validated
                      ↓
    ┌──────────────────────────────────────┐
    │                                      │
[8] Story Engine (Java)            [9] Signal Service (Java)
    Rule-based Telugu/English          Signal lifecycle management
    + Claude AI on-demand              Target/stop price monitoring
    Redis: story:{id}:{lang}           Accuracy tracking
    Kafka: stories.generated           TimescaleDB: signals table
    │                                      │
    └──────────────────────────────────────┘
                      ↓
[10] Alert Service (Java)
     Telegram alerts to users
     Email digests
     Respects user plan + preferences
                      ↓
[11] API Gateway (Java Spring Boot)
     REST endpoints for frontend
     JWT authentication
     Rate limiting per user plan
                      ↓
[12] Frontend (Next.js)
     Stock storyteller UI
     Live scoreboard
     Signal dashboard
     Telugu/English toggle
```

---

## SERVICE 1 — MARKET DATA GATEWAY

Language: Rust
Port: Internal only
Kafka Output: raw.ticks

### Purpose
Connect to broker WebSocket, receive live ticks for 500 NSE stocks,
push to SPSC ring buffer, publish to Kafka.

### What to implement in each file:

**src/main.rs**
- Load config from .env
- Connect to Zerodha WebSocket (primary) or Fyers (fallback)
- Subscribe to 500 instrument tokens loaded from instruments.json
- Start tick receiver thread
- Start Kafka publisher thread
- Health check HTTP endpoint on port 9001

**src/broker/zerodha/mod.rs**
- ZerodhaConnector struct
- connect(api_key, access_token) using Kite WebSocket v2
- subscribe(instrument_tokens: Vec<u32>)
- on_tick callback: parse binary tick format → TickData struct
- Auto-reconnect with exponential backoff on disconnect
- Handle heartbeat pings every 5 seconds

**src/broker/fyers/mod.rs**
- Same interface as ZerodhaConnector
- Also implements: fetch_historical(symbol, tf, from, to) for data loading

**src/ringbuffer/mod.rs**
- SPSCRingBuffer<TickData, 65536>
- Lock-free using atomic head/tail pointers
- push(tick) — called by broker thread (producer)
- pop() → Option<TickData> — called by feed thread (consumer)

**src/feed/mod.rs**
- Reads from ring buffer in tight loop
- Batches 100 ticks → publishes to Kafka in one produce call
- Maintains HashMap<u32, TickData> for latest tick per instrument
- get_latest_tick(instrument_id) → TickData

**src/models/mod.rs**
```
TickData {
  instrument_id: u32,
  ltp: f32,           // last traded price
  volume: u64,
  oi: u64,            // open interest
  bid: f32,
  ask: f32,
  timestamp_ns: u64,
  change_pct: f32,
}
```

---

## SERVICE 2 — CANDLE ENGINE

Language: Rust
Kafka Input: raw.ticks
Kafka Output: candles.1m, candles.5m, candles.15m, candles.1h, candles.4h, candles.1d
TimescaleDB Write: candles table

### Purpose
Transform raw ticks into OHLCV candles for 6 timeframes.
All timeframes built from single tick stream — no duplicate processing.

### What to implement:

**src/timeframes/mod.rs**
- TimeframeState per instrument per TF: open, high, low, close, volume, open_time
- Pre-compute boundary lookup table: for each minute 0-374, which TFs close?
  - 1M: closes every minute
  - 5M: closes at minutes 5,10,15...375
  - 15M: closes at minutes 15,30,45...375
  - 1H: closes at minutes 60,120...360
  - 4H: closes at 240 (only one 4H candle intraday)
  - 1D: closes at 375 (3:30 PM)
- update_tick(tick) → Vec<CompletedCandle> — process tick, return any closed candles
- Reset all states at 9:15 AM daily

**src/storage/mod.rs**
- Ring buffer per instrument per TF: last N candles
  - 1M: 750, 5M: 750, 15M: 750, 1H: 750, 4H: 500, 1D: 750
- get_candles(instrument_id, tf, count) → Vec<Candle>
- Async write to TimescaleDB (non-blocking, batched every 10 candles)

**src/publisher/mod.rs**
- On CompletedCandle: publish to Kafka candles.{tf}
- Update Redis: candle:{instrument_id}:{tf}:latest

---

## SERVICE 3 — INDICATOR ENGINE

Language: Rust + Rayon (parallel)
Kafka Input: candles.{timeframe}
Kafka Output: indicators.snapshot
Redis Write: indicator:{instrument_id}:{tf}

### Purpose
Calculate all 39 indicators for every stock every candle close.
Target: 500 stocks × all indicators in < 200ms using Rayon.

### The 39 Indicators (implement exactly as specified):

#### Wave 1 — No dependencies (fully parallel):
1. EMA(9) — k = 2/(9+1) = 0.2
2. EMA(20) — k = 2/(20+1) = 0.0952
3. EMA(50) — k = 2/(50+1) = 0.0385
4. EMA(200) — k = 2/(200+1) = 0.00995, set ema_200_ready=true after 250 candles
5. SMA(20) — simple average of last 20 closes
6. SMA(50) — simple average of last 50 closes
7. RSI(14) — Wilder smoothing, period=14, needs 28 candles
8. RSI(7) — Wilder smoothing, period=7
9. MACD — fast_ema(12) - slow_ema(26), signal=EMA(9) of MACD, histogram=MACD-signal
10. ATR(14) — Wilder, TR=max(H-L, |H-prevC|, |L-prevC|), period=14
11. ATR(10) — same formula, period=10
12. Bollinger Bands(20,2σ) — upper=SMA20+(2×stddev), lower=SMA20-(2×stddev), width=upper-lower
13. OBV — if close>prev: OBV+=volume, else OBV-=volume
14. OBV_SMA(20) — SMA of OBV over 20 periods
15. OBV_Slope(5) — linear slope of last 5 OBV values
16. Volume Ratio(20) — current_volume / SMA(volume, 20)
17. ROC(12) — (close - close_12_ago) / close_12_ago × 100
18. Stochastic %K(14) — (close - lowest_low_14) / (highest_high_14 - lowest_low_14) × 100
19. Williams %R(14) — (highest_high_14 - close) / (highest_high_14 - lowest_low_14) × -100
20. BB Width — (upper - lower) / middle × 100
21. BB Width Min(20) — minimum BB width over last 20 bars
22. Donchian(20) — dc_upper_20=max(high,20), dc_lower_20=min(low,20)
23. Donchian(252) — dc_upper_252, dc_lower_252 (52-week high/low)
24. Distance to 52W High — (dc_upper_252 - close) / dc_upper_252 × 100
25. Distance from 52W Low — (close - dc_lower_252) / close × 100
26. EMA Stack Score — count(close > ema_9, close > ema_20, close > ema_50, close > ema_200)
27. Consecutive Green/Red — count from current bar backwards

#### Wave 2 — Depend on Wave 1:
28. Supertrend(10,3) — upper_band = (H+L)/2 + 3×ATR(10), lower_band = (H+L)/2 - 3×ATR(10)
    - direction: +1 if close > lower_band, -1 if close < upper_band
    - supertrend_flipped: true if direction changed this bar
    - supertrend_flip_bars_ago: bars since last flip
29. ADX(14) + DI — Wilder smoothed. adx_ready=true after 28 candles
    - +DM = H - prev_H if positive else 0
    - -DM = prev_L - L if positive else 0
    - TR = ATR(14) raw
    - Smooth +DM, -DM, TR with Wilder(14)
    - +DI = 100 × smooth_+DM / smooth_TR
    - -DI = 100 × smooth_-DM / smooth_TR
    - DX = 100 × |+DI - -DI| / (+DI + -DI)
    - ADX = Wilder(14) of DX
30. VWAP — reset at 9:15 AM, VWAP = cumsum(typical_price × volume) / cumsum(volume)
    - typical_price = (H + L + C) / 3
    - std_dev = stddev of (typical_price - VWAP) × volume weighted
    - vwap_upper_1 = VWAP + 1 × std_dev, vwap_lower_1 = VWAP - 1 × std_dev
    - vwap_upper_2 = VWAP + 2 × std_dev, vwap_lower_2 = VWAP - 2 × std_dev
    - vwap_side = +1 if close > vwap else -1
    - vwap_distance_pct = (close - vwap) / vwap × 100
31. CMF(20) — sum(MFV, 20) / sum(volume, 20)
    - MFM = ((close - low) - (high - close)) / (high - low)
    - MFV = MFM × volume
32. Force Index(2) — EMA(2) of (close - prev_close) × volume
33. Stochastic %D(3) — SMA(3) of %K
34. Keltner Channel(20, 2, ATR10) — kc_middle=EMA20, kc_upper=EMA20+(2×ATR10), kc_lower=EMA20-(2×ATR10)
35. BB Squeeze — bb_width < kc_width (Bollinger inside Keltner)
36. Squeeze Released Up — was in squeeze last bar, now bb_width > kc_width AND close > kc_upper
37. Squeeze Released Down — was in squeeze, now close < kc_lower
38. DC Position(20) — (close - dc_lower_20) / (dc_upper_20 - dc_lower_20)

#### Wave 3 — Composite:
39. MTF Alignment Score (0-8) — check all of:
    - 15M: ema_stack >= 2, supertrend = +1 (2 points)
    - 1H: ema_stack >= 2, supertrend = +1 (2 points)
    - 4H: ema_stack >= 2, supertrend = +1 (2 points)
    - 1D: ema_stack >= 3, supertrend = +1 (2 points)

#### Confluence Score (0-100):
- Trend (30pts): EMA stack aligned(15) + Supertrend bullish(10) + above EMA200(5)
- Momentum (25pts): RSI zone 50-70(10) + RSI7>RSI14(5) + MACD hist>0(7) + ADX>25(3)
- Volume (20pts): vol_ratio tiers: >2.5=20, >2.0=16, >1.5=12, >1.0=8, >0.8=4
- Price Action (15pts): VWAP above(8) + above SMA50(4) + BB position upper half(3)
- Market Context (10pts): Nifty above EMA20(5) + sector positive(3) + FII buying(2)
- Bonuses: MTF>=6 add +10, ML probability>70% add +5
- Regime penalties: VOLATILE_CRISIS = force 0, VOLATILE_SPIKE × 0.7

### Implementation structure:

**src/indicators/trend/mod.rs** — implements indicators 1-8, 26-27, 29
**src/indicators/momentum/mod.rs** — implements indicators 7-8, MACD, Stochastic, Williams, ROC, Force Index
**src/indicators/volume/mod.rs** — implements OBV, OBV_SMA, OBV_Slope, Volume Ratio, CMF
**src/indicators/volatility/mod.rs** — implements ATR, BB, Keltner, Squeeze, Donchian
**src/indicators/composite/mod.rs** — implements VWAP, MTF Score, Confluence Score
**src/engine/mod.rs** — orchestrates all waves, Rayon parallel execution

**src/models/mod.rs** — IndicatorSnapshot struct (256-byte aligned):
```
All 39 indicator values as f32
instrument_id: u32
timeframe: u8
timestamp_ns: u64
bars_seen: u16
Flags as u8 packed bits:
  ema_200_ready, adx_ready, bb_squeeze, supertrend_direction
  supertrend_flipped, vwap_side, squeeze_released_up, squeeze_released_down
MTF fields: tf_15m_bullish, tf_1h_bullish, tf_4h_bullish, tf_1d_bullish
confluence_score: u8
mtf_alignment_score: u8
```

---

## SERVICE 4 — SCREENER ENGINE

Language: Rust
Kafka Input: indicators.snapshot
Kafka Output: signals.raw

### Purpose
Run all 30 quantum screeners against IndicatorSnapshots.
Apply deduplication. Build validated trade setups. Emit signals.

### All 30 Screeners — Conditions Summary:

**Trend Screeners:**
- SCR01 (52W Breakout): close > dc_upper_252 × 0.998, vol>=2.0x, rsi>55, ema_stack>=3, close>ema_200
- SCR02 (Supertrend Flip): supertrend_flipped=true, bars_ago<=2, rsi>50, macd_hist>0, vol>=1.5x
- SCR04 (EMA Pullback): price near ema_20(within 0.5×ATR), ema_20 rising, rsi 40-55, vol<1.2x on pullback
- SCR14 (HHHL): 2 confirmed HH+HL, at last HL zone(within 1.5%), bounce candle, rsi 38-55
- SCR15 (Golden Cross): ema_20 crosses above ema_50 within 3 bars, both rising, vol>=2.0x, rsi 50-72
- SCR29 (ADX Surge): adx<18 five_bars_ago, adx>=20 now, risen>=5pts, +DI>-DI, dc_upper_20 break
- SCR30 (High Tight Flag): doubled in 40 days, flag<25% correction, 10-25 days, vol on breakout>=2.5x

**Accumulation Screeners:**
- SCR13 (Wyckoff): PhaseD detected OR spring within 5 bars, AR_high breakout, vol>=2.0x, cmf>0
- SCR26 (OBV Divergence): OBV up 5%+ while price flat 10 bars, cmf>0.05, price trigger (ema20 cross)
- SCR28 (Rounding Bottom): 40+ day U-shape, volume mirrors shape, breakout above rim on vol>=2.5x

**Candlestick Screeners:**
- SCR16 (Bullish Engulfing): full engulf, at support zone, consecutive_red>=2, rsi 25-50, vol>=1.5x
- SCR17 (Morning Star): 3-candle pattern (large red→doji→large green), at support, volume pattern
- SCR18 (Three White Soldiers): 3 green, each in prior body, closes near high, vol rising, rsi<70

**Chart Pattern Screeners:**
- SCR05 (BB Squeeze): squeeze_released_up=true, squeeze duration>=5, vol>=2.0x, rsi>55
- SCR09 (Inside Bar): current range inside prior range, breakout on vol>=1.5x
- SCR19 (Cup & Handle): cup 20-60 days 12-35% depth, handle 5-20 days <15%, breakout on vol>=2.5x
- SCR20 (Ascending Triangle): flat resistance 2+ touches, rising lows, breakout on vol>=2.0x
- SCR21 (Flag/Pennant): 5%+ pole in 3-5 days, 5-15 day flag, declining vol, breakout on vol>=2.0x
- SCR27 (Double Bottom): two lows within 2%, RSI divergence, neckline breakout on vol>=2.0x

**Volume/Momentum Screeners:**
- SCR03 (VWAP Reclaim): cross above VWAP, vol>1.5x, only 9:30-1:30 PM
- SCR06 (RSI Divergence): price lower lows, RSI higher lows, 2+ swing points confirmed
- SCR07 (Volume Climax): vol_ratio>=4.0, at support, long lower wick, close>open
- SCR08 (ORB): ORB set 9:15-9:30 AM, break above orb_high on vol>=2.0x before 11 AM
- SCR10 (Perfect Storm): 6+ conditions bullish: ema_stack=4, supertrend=+1, rsi>55, macd>0, vol>2x, vwap_above
- SCR11 (Demand Zone): price at demand zone, bounce candle, rsi<50, confluence>=70
- SCR12 (MTF Surge): mtf_alignment_score>=6, all major TFs bullish

**Institutional Screeners:**
- SCR22 (FII Confluence): fii_net_buyers_3d=true, fii>=500cr, price trigger, sector positive, mktcap>5000cr
- SCR24 (Sector Rotation): sector up>1.5% today, was negative last week, stock is sector leader
- SCR25 (Earnings Gap): 3%+ gap, held 3+ days, tight consolidation, continuation on vol>=1.8x

**Special Screeners:**
- SCR23 (Max Pain): only expiry week, price 1.5%+ from max_pain, direction toward max_pain

### Trade Setup Rules (apply to ALL screeners):
- entry_ideal, entry_zone_low, entry_zone_high from screener-specific formula
- stop_loss must be 0.3% to 5% of entry (reject if outside range)
- R:R to Target 2 must be >= 2.0 (reject if below)
- Volume ratio at signal time must be >= 0.8
- Market hours: 9:15 AM to 3:20 PM only
- Position sizing: risk_amount = account_capital × risk_pct / 100
  quantity = risk_amount / (entry - stop_loss)
  capital_required = quantity × entry

### Dedup Rules:
- Per screener cooldown: see individual screener specs above
- Same stock + any screener: minimum 5-minute gap
- Same stock: maximum 2 signals per trading day
- Same sector: maximum 4 signals per hour
- VOLATILE_CRISIS: block all signals
- VOLATILE_SPIKE: only SCR10 allowed

### Daily Output:
- Maximum 10 signals total
- Sort by confluence_score descending
- Priority tiebreaker: SCR10 > SCR30 > SCR13 > SCR26 > SCR22 > SCR14

---

## SERVICE 5 — SCOREBOARD ENGINE

Language: Rust
Kafka Input: indicators.snapshot
Kafka Output: signals.raw (when score >= 70 and eligible)
Redis Write: scoreboard:all (full 500-stock scoreboard)

### Scoring Architecture:

**Bull Score per timeframe (0-100):**
```
TREND (max 35pts):
  close > ema_9: +5
  close > ema_20: +5
  close > ema_50: +7
  close > ema_200: +8
  ema_9 > ema_20 > ema_50 (stack aligned): +5
  supertrend_direction == +1: +5

MOMENTUM (max 25pts):
  rsi_14 between 50-70: +8
  rsi_7 > rsi_14: +4
  macd_histogram > 0: +5
  macd_histogram > prev_histogram: +4
  roc_12 > 2.0: +4

VOLUME (max 20pts):
  volume_ratio > 1.5: +7
  volume_ratio > 2.5 (bonus): +5
  obv_slope_5 > 0: +5
  cmf_20 > 0.05: +3

STRUCTURE (max 20pts):
  close > vwap (intraday TFs only): +5
  bb_squeeze == true: +4
  squeeze_released_up: +6
  dc_position_20 > 0.7: +5
  adx > 25 AND plus_di > minus_di: +5
```

**Bear Score per timeframe (0-100):**
Mirror of bull with inverted conditions.

**Timeframe Weights:**
1M=5%, 5M=10%, 15M=20%, 1H=30%, 1D=35%

**MTF Alignment Bonus:**
5/5 TFs bull>=50: +15
4/5 TFs bull>=50: +10
3/5 TFs bull>=50: +5
Cap total at 100.

**Eligibility for signal fire:**
BULL: bull_1d>=50 AND bull_1h>=45 AND bull_total>=60 AND bull>bear+15 AND vol>=1.0 AND close>ema_200_1d
BEAR: mirror conditions
FIRE: score>=80 → ELITE, 70-79 → HIGH, 60-69 → MEDIUM (watchlist only)

---

## SERVICE 6 — ML SERVICE

Language: Python 3.11
Framework: FastAPI (HTTP) + scheduled batch jobs
Port: 8001

### Models to implement:

**XGBoostDirectionPredictor**
- 47 input features (39 indicators + 8 contextual)
- Output: bull_probability (0-1), bear_probability (0-1)
- Training label: price rises >2% within 5 trading days
- Minimum accuracy to deploy: 62%
- Retrain: every Sunday 9 PM on MacBook M2

**LightGBMSignalQualityScorer**
- Features: signal type + all indicators + market context
- Output: quality_score (0-1) — how clean was this signal historically
- Training: historical signals database + outcomes

**LSTMPatternMatcher**
- Input: 60-day sequence of [close, vol, rsi, macd, ema_ratio, bb_pos, adx, vwap_dist]
- Shape: (batch, 60, 8) normalized
- Output: pattern_score (0-100), next_5d_return_prediction
- Architecture: 2-layer LSTM (128 hidden) → Dense(64) → Output(2)
- Training: PyTorch with MPS backend

**RandomForestRegimeClassifier**
- Input: Nifty50 indicators + VIX + FII + sector breadth
- Output: 7-class regime (BULL_TRENDING to CRISIS)
- Accuracy target: 78-84%

**IsolationForestAnomalyDetector**
- Unsupervised, no labels needed
- Input: 30-day volume+price behavior per stock
- Output: anomaly_score, anomaly_type

### Batch inference (9:00 AM daily):
1. Load all 5 models into memory
2. Fetch indicator snapshots for all 500 stocks from Redis
3. Build feature matrix (500 × 47)
4. Run XGBoost: batch predict_proba → 500 × 2 probabilities
5. Run LightGBM: batch predict → 500 quality scores
6. Run LSTM: batch inference → 500 pattern scores
7. Run Regime: single prediction for market
8. Run Anomaly: 500 scores
9. Write all results to Redis ml:{id}:scores with 24h TTL
10. Total target time: < 2 minutes

### HTTP endpoints:
- POST /ml/inference/{instrument_id} — on-demand single stock
- GET /ml/regime — current market regime
- GET /ml/accuracy — model accuracy stats
- POST /ml/retrain — trigger retraining (admin only)

---

## SERVICE 7 — STORY ENGINE

Language: Java 21 Spring Boot 3.2
Port: 8002
Kafka Input: indicators.snapshot, signals.validated
Redis Read: indicator:{id}:{tf}, ml:{id}:scores
Redis Write: story:{id}:{lang} (TTL: 15 minutes)

### Story Types:

**RULE_BASED (free, instant — all 500 stocks):**
- Select appropriate template sentences based on indicator conditions
- Fill {variable} placeholders with actual values
- Assemble 8 chapters
- Available in English and Telugu immediately
- Regenerate only if score changes > 5 points (saves compute)

**AI_ENHANCED (Claude Haiku — top 50 stocks by score change):**
- Generate rule-based story first
- Enhance with Claude Haiku API call
- Richer language, better flow, contextual awareness
- Cache in Redis 15 minutes

**ON_DEMAND (Claude Sonnet — user clicks stock):**
- Triggered by API call from frontend
- Full deep analysis prompt sent to Claude Sonnet
- All 8 chapters generated by Claude in one call
- Telugu translation in same call if user preference is Telugu
- Cache in Redis 15 minutes (prevents duplicate calls)

### Template Condition Rules:

Chapter 1 (Big Picture) conditions:
```
close > ema_200 AND ema_200 rising → ABOVE_EMA200_STRONG template
close > ema_200 AND ema_200 flat → ABOVE_EMA200_FLAT template
close < ema_200 AND ema_200 falling → BELOW_EMA200_FALLING template
|close - ema_200| < 0.5% → AT_EMA200_DECISION template
close > ema_200 AND fresh cross (5 bars) → FRESH_EMA200_CROSS template
```

Chapter 2 (Momentum) conditions:
```
ema_9 > ema_20 > ema_50 > ema_200 → EMA_STACK_PERFECT_BULL
ema_9 > ema_20 but ema_20 < ema_50 → EMA_STACK_PARTIAL_BULL
ema_9 < ema_20 < ema_50 → EMA_STACK_BEARISH
rsi between 50-65 AND rsi_7 > rsi_14 → RSI_HEALTHY_ACCELERATING
rsi > 70 → RSI_OVERBOUGHT_CAUTION
rsi < 30 → RSI_OVERSOLD_BOUNCE_WATCH
rsi_divergence_bullish → RSI_DIVERGENCE_BULLISH
macd_histogram > 0 AND rising → MACD_ACCELERATING_BULL
macd_histogram < 0 AND rising (recovery) → MACD_RECOVERING
```

Chapter 3 (Volume) conditions:
```
volume_ratio >= 3.0 AND green → VOLUME_INSTITUTIONAL_SURGE
volume_ratio < 0.7 AND green → VOLUME_LOW_CAUTION
obv rising AND price flat → OBV_ACCUMULATION_SILENT
cmf > 0.15 → CMF_STRONG_INFLOW
cmf < -0.15 → CMF_STRONG_OUTFLOW
```

Chapter 5 (Volatility) conditions:
```
bb_squeeze AND squeeze_duration >= 5 → SQUEEZE_BUILDING
squeeze_released_up → SQUEEZE_EXPLODING_UP
atr_percentile > 80 → HIGH_VOLATILITY_CAUTION
atr_percentile < 20 → CALM_BEFORE_STORM
```

Chapter 8 (Verdict) conditions:
```
bull_total >= 80 → VERDICT_ELITE_BULL (with full trade setup)
bull_total 70-79 → VERDICT_STRONG_BULL
bull_total 60-69 → VERDICT_WEAK_BULL (watchlist)
bull_total and bear_total both 40-60 → VERDICT_CONFLICTED_WAIT
bear_total >= 80 → VERDICT_ELITE_BEAR
```

### Telugu Templates:
All templates have Telugu equivalents in TeluguTemplates.java.
Telugu style: conversational Hyderabad dialect.
Keep in English: RSI, MACD, EMA, ATR, VWAP, all stock names, all prices.

### Claude API Prompt (ON_DEMAND):
```
System: "You are a senior stock market analyst explaining {symbol}
to a retail investor in {language}. Use simple language with analogies.
Be specific with numbers. Be honest about risks. Not a financial advisor disclaimer at the end.
For Telugu: use conversational Hyderabad style, keep all indicator names and prices in English."

User: "Analyse this stock and tell the story in 8 chapters:
[full indicator JSON + ML scores + market context]

Format: 8 labeled sections (Chapter 1 through 8).
Each chapter: 3-5 sentences maximum.
Chapter 8 must include: trade setup card formatted as a table."
```

---

## SERVICE 8 — AI VALIDATION SERVICE

Language: Java 21 Spring Boot 3.2 (Spring WebFlux for async)
Port: 8003
Kafka Input: signals.raw
Kafka Output: signals.validated

### Validation Flow:
1. Receive TradingSignal from Kafka
2. Check events_risk (earnings/RBI/budget) → auto-reject if true
3. Check ML probability — if xgb_bull_prob < 55% → reject with reason
4. Build validation prompt (ValidationPromptBuilder)
5. Call Claude Haiku API (async, timeout 3 seconds)
6. Parse structured response: VERDICT / CONFIDENCE / REASON / RED_FLAGS
7. If APPROVE: publish to signals.validated
8. If REJECT: log with reason, publish to signals.rejected (for ML training)
9. If Claude unavailable: approve if confluence_score > 82 (fallback rule)

### Validation Prompt:
Include all of these in prompt:
- Symbol, direction, entry/stop/target/RR
- Top 10 most relevant indicators (not all 39 — summarize)
- ML scores (XGBoost, LightGBM, LSTM probabilities)
- Market context (Nifty direction, VIX, FII status, sector)
- Regime (current market regime from ML)
- Events risk flag
- Which screener fired + brief rule description

Expected response format (strict):
```
VERDICT: APPROVE
CONFIDENCE: HIGH
REASON: Two sentences maximum.
RED_FLAGS: NONE or comma-separated flags
TELUGU: Telugu translation of verdict + reason
```

### Cost Management:
- Use claude-haiku-4-5-20251001 only for validation
- Prompt caching: system prompt cached (saves 60% input tokens)
- Only validate signals with confluence_score >= 60 (skip weak signals)
- Rate limit: max 20 concurrent calls

---

## SERVICE 9 — SIGNAL SERVICE

Language: Java 21 Spring Boot 3.2
Port: 8004
Database: TimescaleDB
Kafka Input: signals.validated
Kafka Output: signals.updates, alerts.pending

### Signal Lifecycle:
```
ACTIVE → price hits T1 → T1_HIT (partial exit)
       → price hits T2 → T2_HIT
       → price hits T3 → T3_HIT (full exit)
       → price hits stop → STOPPED
       → valid_until passed → EXPIRED
```

### Price Monitoring:
- Subscribe to live tick feed (Redis: candle:{id}:latest polling every 30 seconds)
- For each active signal: check if high >= target or low <= stop_loss
- On target hit: update status, calculate actual_return, publish update
- Accuracy: wins/total per screener, per regime, per sector
- Weekly report: accuracy summary for all screeners

### REST Endpoints:
- GET /signals?status=ACTIVE&date=today
- GET /signals/{id}
- GET /signals/history?days=30
- GET /signals/accuracy/screeners
- GET /signals/accuracy/summary
- POST /signals/{id}/paper-trade — user marks as paper traded
- GET /signals/performance — P&L tracker if user enters actual trades

---

## SERVICE 10 — ALERT SERVICE

Language: Java 21 Spring Boot 3.2
Port: 8005
Kafka Input: signals.validated, stories.generated
External: Telegram Bot API, SMTP email

### Alert Types:
1. **Signal Alert** (immediate when signal fires):
   ```
   🟢 STRONG BULL — {SYMBOL}
   Score: {score}/100 | Screener: {screener_name}
   Entry: ₹{entry} | Stop: ₹{stop} (-{stop_pct}%)
   T1: ₹{t1} | T2: ₹{t2} | R:R: 1:{rr}
   {2-sentence Telugu/English story snippet}
   ```

2. **Morning Briefing** (9:00 AM daily):
   - Top 5 bull setups for today
   - Market regime + Nifty outlook
   - FII/VIX summary
   - Telugu/English based on user preference

3. **EOD Summary** (3:40 PM daily):
   - Which signals hit targets/stops
   - Today's win rate
   - Tomorrow's watchlist

4. **Weekly Report** (Sunday 7 PM):
   - Week's performance summary
   - Best performing screeners
   - Sector rotation outlook

### User Preferences:
- Language: EN / TE
- Alert types: which of the 4 types to receive
- Quiet hours: default 10 PM - 9 AM (no alerts)
- Plan-based filtering: Basic gets morning brief only, Pro gets all alerts, Elite gets priority

---

## SERVICE 11 — API GATEWAY

Language: Java 21 Spring Boot 3.2
Port: 8080 (public-facing, behind Nginx)
Auth: JWT validation on all protected routes

### API Endpoints:

**Stocks:**
- GET /api/v1/stocks — paginated list with scoreboard (public preview: top 10)
- GET /api/v1/stocks/top/bull — top 20 bull scores
- GET /api/v1/stocks/top/bear — top 20 bear scores
- GET /api/v1/stocks/{symbol} — full stock data
- GET /api/v1/stocks/{symbol}/story?lang=te — story (cached 15min)
- GET /api/v1/stocks/{symbol}/story/deep — AI deep analysis (on-demand Claude)
- GET /api/v1/stocks/{symbol}/indicators — indicator snapshot
- GET /api/v1/stocks/{symbol}/sr-levels — S/R zones
- GET /api/v1/stocks/search?q={query} — search by symbol/name

**Signals:**
- GET /api/v1/signals — today's signals (PRO plan required)
- GET /api/v1/signals/{id} — signal detail
- GET /api/v1/signals/history — past signals with outcomes
- GET /api/v1/signals/accuracy — screener accuracy stats

**Market:**
- GET /api/v1/market/regime — current regime
- GET /api/v1/market/context — Nifty, VIX, FII, sectors

**User:**
- GET /api/v1/user/profile
- PUT /api/v1/user/preferences (language, alert settings)
- GET /api/v1/user/watchlist
- POST /api/v1/user/watchlist/{symbol}

**Auth:**
- POST /api/v1/auth/register
- POST /api/v1/auth/login
- POST /api/v1/auth/refresh
- POST /api/v1/auth/logout

**Subscription:**
- GET /api/v1/subscription/plans
- POST /api/v1/subscription/subscribe
- GET /api/v1/subscription/status
- POST /api/v1/subscription/cancel

### Rate Limiting:
- BASIC plan: 60 requests/minute
- PRO plan: 200 requests/minute
- ELITE plan: 500 requests/minute
- Unauthenticated: 10 requests/minute

---

## SERVICE 12 — AUTH SERVICE

Language: Java 21 Spring Boot 3.2
Port: 8006
Database: TimescaleDB (users table)

### Implementation:
- Registration: validate email uniqueness, bcrypt password, send verification email
- Login: verify credentials, issue JWT (24h) + refresh token (30d)
- JWT: RS256 signed, contains user_id + plan + language_preference
- Refresh: validate refresh token from Redis, issue new JWT
- Logout: blacklist JWT in Redis until expiry
- Password reset: email OTP flow

---

## SERVICE 13 — SUBSCRIPTION SERVICE

Language: Java 21 Spring Boot 3.2
Port: 8007
Database: TimescaleDB (subscriptions table)
Payment: Razorpay integration

### Plans:
```
BASIC ₹499/month:
  - Rule-based stories only (no Claude AI)
  - 5 signals per day
  - English only (no Telugu AI)
  - Morning briefing alert only
  - No deep analysis

PRO ₹999/month:
  - AI-enhanced stories (Haiku)
  - All signals (up to 10/day)
  - Telugu + English
  - All alert types
  - On-demand deep analysis (5/day limit)

ELITE ₹1999/month:
  - Everything in PRO
  - On-demand deep analysis (unlimited)
  - Weekly deep sector reports
  - Priority signal delivery
  - Monthly health report cards
  - WhatsApp alerts (if implemented)
```

### Razorpay Integration:
- Create subscription plan in Razorpay dashboard
- POST /subscription/subscribe → create Razorpay subscription → return payment link
- Webhook /subscription/webhook → handle payment.captured, subscription.charged, subscription.cancelled
- Auto-downgrade to FREE on subscription end

---

## FRONTEND — Next.js 14

Language: TypeScript
Styling: Tailwind CSS
State: Zustand
API Calls: Axios + React Query
Real-time: WebSocket (Socket.io)
Charts: TradingView Lightweight Charts

### Pages:

**/ (Landing)**
- Marketing page, pricing, sample story demo
- No auth required

**/app/dashboard (Protected)**
- Market regime banner (full width)
- Top 5 bull + bear stocks as scoreboard cards
- Today's signal feed (live updates)
- Morning briefing story card
- Telugu/English toggle top right

**/app/stocks (Protected)**
- All 500 stocks sortable table
- Columns: Symbol, Bull Score, Bear Score, Signal, Trend
- Live score updates every 1 minute
- Filter by sector, score range, signal status
- Click → /app/stocks/[symbol]

**/app/stocks/[symbol] (Protected)**
- LEFT PANEL: Scoreboard widget (per-TF scores as bars)
- CENTER: Story chapters as reading cards (8 chapters)
- RIGHT PANEL: Trade setup card + What To Watch
- BOTTOM: Multi-TF alignment visual
- Language toggle: instant switch EN↔TE
- "Deep Analysis" button (PRO/ELITE only)

**/app/signals (Protected — PRO+)**
- Signal cards with status indicators
- Filter: Active / All / By screener
- Each card: entry/stop/target visual, Claude verdict badge

**/app/settings**
- Language preference
- Alert preferences
- Telegram bot connection
- Subscription management

### Key Components:

**StoryCard.tsx**
- Props: chapter_number, title, icon, text, language, score
- Typewriter animation for AI-generated stories
- Score badge (colored by value)
- Expandable on mobile

**ScoreboardWidget.tsx**
- Per-TF bull/bear score horizontal bars
- Green for bull, red for bear, grey for neutral
- Animated on data update
- Total score large display center

**SignalCard.tsx**
- Price ladder visual: entry → T1 → T2 → T3
- Stop loss below entry
- R:R ratio badge
- Claude verdict badge: ✅ APPROVED / ⚠️ WATCHLIST
- Status badges: 🟢 ACTIVE / 🎯 T1 HIT / ✋ STOPPED

**MarketContextBar.tsx**
- Top bar on all pages
- Nifty: price + change + regime pill
- VIX: value + trend arrow
- FII: buying/selling + amount
- Sector rotation: top 3 sectors today

---

## DATABASE SCHEMA OVERVIEW

TimescaleDB Tables (full schema in infrastructure/timescaledb/init.sql):

**candles** — hypertable, partition by day
  instrument_id, timeframe, open_time (partition key), open, high, low, close, volume

**indicators** — hypertable, partition by day
  instrument_id, timeframe, timestamp (partition key), all 39 indicator columns

**signals** — hypertable, partition by month
  signal_id, instrument_id, screener_id, created_at (partition key)
  entry, stop, t1, t2, t3, status, outcome, actual_return
  claude_verdict, confluence_score, ml_xgb_prob

**signal_outcomes** — accuracy tracking
  signal_id, outcome, actual_return_pct, hold_duration_bars, regime_at_signal

**stocks** — static table
  instrument_id, symbol, exchange, sector_id, market_cap_cr, lot_size

**users** — static table
  id, email, password_hash, name, phone, language_pref, created_at

**subscriptions** — static table
  user_id, plan, start_date, end_date, razorpay_subscription_id, status

**stories** — rolling 7-day retention
  instrument_id, generated_at, story_type, en_story, te_story, bull_score, bear_score

---

## REDIS KEY PATTERNS

```
candle:{instrument_id}:{tf}:latest          → latest CompletedCandle JSON   TTL: 2min
indicator:{instrument_id}:{tf}              → IndicatorSnapshot JSON         TTL: 5min
scoreboard:all                              → full 500-stock scoreboard       TTL: 1min
scoreboard:{instrument_id}                  → single stock scoreboard         TTL: 1min
ml:{instrument_id}:scores                   → ML inference results            TTL: 24h
story:{instrument_id}:en                    → English story                   TTL: 15min
story:{instrument_id}:te                    → Telugu story                    TTL: 15min
signal:{signal_id}                          → TradingSignal JSON              TTL: 7d
signal:active:{instrument_id}               → active signal for stock         TTL: 7d
session:{user_id}                           → user session data               TTL: 24h
jwt:blacklist:{jti}                         → blacklisted JWT                 TTL: 24h
rate:limit:{user_id}:{minute}              → request count                   TTL: 1min
market:regime                               → current regime string           TTL: 1h
market:context                              → Nifty/VIX/FII JSON             TTL: 5min
```

---

## KAFKA TOPICS

```
raw.ticks           partitions=10   retention=1h    — raw tick data
candles.1m          partitions=5    retention=2d    — 1-minute candles
candles.5m          partitions=5    retention=7d
candles.15m         partitions=5    retention=7d
candles.1h          partitions=5    retention=30d
candles.4h          partitions=5    retention=30d
candles.1d          partitions=3    retention=365d
indicators.snapshot partitions=10   retention=1d    — indicator snapshots
signals.raw         partitions=3    retention=7d    — pre-validation signals
signals.validated   partitions=3    retention=30d   — approved signals
signals.rejected    partitions=3    retention=7d    — rejected (for ML training)
signals.updates     partitions=3    retention=30d   — lifecycle updates
stories.generated   partitions=3    retention=7d    — generated stories
alerts.pending      partitions=3    retention=1d    — queued alerts
```

---

## MARKET DATA REQUIREMENTS

### Candle Records Needed:
```
Timeframe   Records   Coverage           Purpose
1D          750       ~3 years           EMA200, DC252, all patterns
1H          500       ~80 trading days   MTF analysis, swing patterns
4H          500       ~500 trading days  Medium-term trend
15M         500       ~20 trading days   VWAP, intraday trend
5M          500       ~7 trading days    Intraday momentum
1M          750       ~2 trading days    Precise entry timing
```

### Data Sources:
- Historical: Fyers API (25 years free, load at startup)
- Live: Zerodha Kite WebSocket (primary) / Fyers WebSocket (fallback)
- Fundamentals: Screener.in free API
- News: Finnhub free tier (60 req/min)
- FII/DII: NSE website scraping (daily at 6 PM)
- Options chain: NSE website (for Max Pain — expiry week only)
- India VIX: NSE website (every 15 minutes)

---

## DEVELOPMENT ORDER (Build in this sequence)

### Phase 1 — Foundation (Month 1):
1. Infrastructure setup: TimescaleDB + Redis + Kafka
2. Historical data loader script
3. Market Data Gateway (Rust) — connect to broker, write ticks to Kafka
4. Candle Engine (Rust) — build candles, write to DB
5. First 10 indicators (EMA, SMA, RSI, MACD, ATR, BB)
6. First 5 screeners (SCR01, SCR02, SCR10, SCR03, SCR08)
7. Auth Service + basic API Gateway
8. Simple Next.js frontend with signal display
9. Telegram alerts for signals

### Phase 2 — Core Analysis (Month 2):
10. All 39 indicators
11. All 30 screeners + DedupEngine
12. Scoreboard Engine
13. Rule-based Story Engine (templates only, no Claude)
14. Signal Service (lifecycle tracking)
15. Frontend: stock story pages

### Phase 3 — Intelligence (Month 3):
16. ML Service: XGBoost + LightGBM (batch inference)
17. AI Validation Service (Claude Haiku)
18. Story Engine: Claude AI stories
19. Telugu translation (templates + Claude)
20. Subscription Service + Razorpay
21. Frontend: full storyteller UI

### Phase 4 — Refinement (Month 4-6):
22. LSTM model training + integration
23. Regime detection model
24. Anomaly detection
25. S/R zone detection
26. Price action analysis (Order Blocks, FVG, Liquidity)
27. Performance optimization (< 200ms indicator calculation)
28. Accuracy tracking + feedback loop
29. Mobile-responsive frontend
30. Beta users + refinement based on real data

---

## CODING STANDARDS

### Rust Services:
- Use anyhow for error handling
- Tracing crate for logging
- Tokio for async runtime
- Clippy warnings as errors
- All public functions must have doc comments
- Test coverage target: 60% for business logic

### Java Services:
- Spring Boot 3.2 with Virtual Threads (Java 21)
- ZGC garbage collector
- No blocking calls on reactor thread (WebFlux)
- All entities use JPA + Hibernate
- Flyway for DB migrations
- Actuator endpoints for health checks
- Lombok for boilerplate reduction

### Python:
- Type hints everywhere
- Pandas for data manipulation
- NumPy for math
- FastAPI for HTTP
- Pydantic for request/response models
- Black formatter
- Pytest for tests

### Frontend:
- TypeScript strict mode
- Component: one file per component
- No inline styles (Tailwind only)
- React Query for all server state
- Zustand for client state
- All API calls go through lib/api/ files

---

## ENVIRONMENT SETUP

### Local Development:
```
1. Clone repo
2. Copy .env.example to .env, fill credentials
3. docker-compose up timescaledb redis kafka
4. python scripts/data-loader/load_historical.py --symbols test10
5. Start services in order: MDG → Candle → Indicator → Screener → Scoreboard
6. Start Java services: auth → api-gateway → story → signal
7. npm run dev in frontend/
```

### VPS Deployment:
```
1. Run scripts/setup/setup_vps.sh
2. Fill .env with production credentials
3. docker-compose -f docker-compose.prod.yml up -d
4. Monitor via Grafana at :3001
```

---

## PERFORMANCE TARGETS

```
Market Data Gateway:     < 5ms tick → Kafka (Zerodha WS latency)
Candle Engine:           < 1ms tick → candle update
Indicator Engine:        < 200ms for all 500 stocks × all indicators
Screener Engine:         < 100ms for all 30 screeners × 500 stocks
Scoreboard Engine:       < 50ms for all 500 stocks
ML Batch Inference:      < 2 minutes for all 500 stocks
AI Validation (Claude):  < 3 seconds per signal
Story Generation:        < 100ms rule-based, < 3s Claude-enhanced
API Response:            < 200ms p95 (cached data)
Frontend Load:           < 2 seconds (LCP)
```

---

## MONITORING AND ALERTING

Prometheus metrics to expose from each service:
- ticks_received_total, ticks_per_second
- candles_closed_total (per timeframe)
- indicator_calculation_duration_ms
- screener_signals_fired_total (per screener)
- signal_win_rate (per screener, rolling 20 trades)
- ml_inference_duration_ms, ml_accuracy_current
- claude_api_calls_total, claude_api_cost_usd_total
- api_requests_total, api_latency_ms (p50, p95, p99)
- active_users, active_subscriptions

Grafana dashboards:
- System health (all service status)
- Signal performance (win rates per screener)
- ML model accuracy tracking
- Claude API cost tracking
- User analytics (signups, churn, plan distribution)

Alert rules (PagerDuty/Telegram to admin):
- Any Rust service down → immediate alert
- Signal win rate drops below 55% for 20+ trades
- Claude API cost exceeds ₹500/day
- API latency p95 exceeds 1 second
- TimescaleDB disk > 80%

---

## COST SUMMARY (Monthly)

```
VPS Hostinger:          ₹2,000
ML electricity:         ₹32
Claude API (optimized): ₹1,200 - ₹4,000 (depends on users)
Domain + SSL:           ₹150
Email service:          ₹200
Total:                  ₹3,582 - ₹6,382/month

Break-even: 4-7 paying users at ₹999/month
Target Month 6: 150 Pro users = ₹1,49,850 revenue
Target Month 12: 500 users mixed = ₹4,00,000+ revenue
```

---

## IMPORTANT: WHAT NOT TO DO

- DO NOT use localStorage or sessionStorage (not supported in Claude artifacts)
- DO NOT store API keys in frontend code
- DO NOT skip the 5-gate signal quality filter
- DO NOT fire signals during VolatileCrisis regime
- DO NOT skip deduplication — it prevents signal spam
- DO NOT use blocking calls in Spring WebFlux reactive pipelines
- DO NOT deploy ML model with accuracy below 62%
- DO NOT call Claude API for every stock every refresh (use caching)
- DO NOT trade on Budget day, RBI policy day, or stock earnings day
- DO NOT move stop losses further after setting them
- DO NOT remove the minimum R:R 2.0 validation gate

---

## QUESTIONS TO RESOLVE BEFORE CODING EACH SERVICE

Before coding any service, answer:
1. Which Kafka topics does this service consume?
2. Which Kafka topics does this service produce?
3. What does it read from Redis?
4. What does it write to Redis?
5. What does it read/write to TimescaleDB?
6. What are the failure modes and how to handle them?
7. What metrics does it expose to Prometheus?
8. What is the expected latency under normal load?

If you cannot answer all 8 questions, read this document again before coding.
