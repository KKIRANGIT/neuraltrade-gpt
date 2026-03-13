export type Language = "en" | "te";
export type Plan = "FREE" | "BASIC" | "PRO" | "ELITE";
export type SignalDirection = "BULL" | "BEAR";
export type SignalStatus =
  | "ACTIVE"
  | "WATCHLIST"
  | "T1_HIT"
  | "T2_HIT"
  | "T3_HIT"
  | "STOPPED"
  | "EXPIRED";
export type ClaudeVerdict = "APPROVE" | "WATCHLIST" | "REJECT";
export type TimeframeKey = "1M" | "5M" | "15M" | "1H" | "4H" | "1D";

export interface TimeframeScore {
  bull: number;
  bear: number;
  trend: "bullish" | "neutral" | "bearish";
}

export interface StoryChapter {
  chapter: number;
  title: string;
  titleTe: string;
  icon: string;
  score: number;
  text: Record<Language, string>;
}

export interface TradeSetup {
  entry: number;
  zoneLow: number;
  zoneHigh: number;
  stop: number;
  targets: [number, number, number];
  rr: number;
  validUntil: string;
}

export interface IndicatorSnapshot {
  close: number;
  ema9: number;
  ema20: number;
  ema50: number;
  ema200: number;
  rsi14: number;
  rsi7: number;
  macdHistogram: number;
  volumeRatio: number;
  obvSlope5: number;
  cmf20: number;
  adx: number;
  plusDi: number;
  minusDi: number;
  bbSqueeze: boolean;
  supertrendDirection: 1 | -1;
  vwapDistancePct: number;
  mtfAlignment: number;
  confluenceScore: number;
}

export interface SignalRecord {
  id: string;
  symbol: string;
  company: string;
  direction: SignalDirection;
  screenerId: string;
  screenerName: string;
  verdict: ClaudeVerdict;
  status: SignalStatus;
  confluenceScore: number;
  entry: number;
  stop: number;
  targets: [number, number, number];
  rr: number;
  createdAt: string;
  snippet: Record<Language, string>;
}

export interface StockRecord {
  instrumentId: number;
  symbol: string;
  name: string;
  sector: string;
  marketCapCr: number;
  price: number;
  changePct: number;
  bullScore: number;
  bearScore: number;
  trend: "Bullish" | "Neutral" | "Bearish";
  signalStatus: SignalStatus | "NONE";
  signalLabel: string;
  confluenceScore: number;
  setup: TradeSetup;
  whatToWatch: string[];
  supportResistance: number[];
  indicators: IndicatorSnapshot;
  scoreboard: Record<TimeframeKey, TimeframeScore>;
  story: StoryChapter[];
  signal?: SignalRecord;
}

export interface MarketContext {
  regime: string;
  nifty: {
    price: number;
    changePct: number;
    outlook: string;
  };
  vix: {
    value: number;
    trend: "Rising" | "Cooling";
  };
  fii: {
    flowCr: number;
    status: string;
  };
  topSectors: string[];
}

export interface UserProfile {
  name: string;
  email: string;
  language: Language;
  plan: Plan;
  watchlist: string[];
  quietHours: string;
  alertTypes: string[];
  telegramConnected: boolean;
}

export interface SubscriptionPlan {
  id: Plan;
  price: string;
  highlight: string;
  features: string[];
}
