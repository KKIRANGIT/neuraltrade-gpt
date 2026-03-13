import {
  type Language,
  type MarketContext,
  type SignalRecord,
  type StockRecord,
  type StoryChapter,
  type SubscriptionPlan,
  type UserProfile
} from "@/lib/types";

function buildStory(
  symbol: string,
  sector: string,
  bullScore: number,
  signalLabel: string
): StoryChapter[] {
  return [
    {
      chapter: 1,
      title: "Big Picture",
      titleTe: "పెద్ద చిత్రం",
      icon: "Compass",
      score: bullScore,
      text: {
        en: `${symbol} is trading above its long-term structure, and the broader ${sector} tape is still constructive. The current move looks like expansion from a healthy base rather than late-stage panic buying.`,
        te: `${symbol} long-term structure paina undi, ${sector} sector tone kuda supportive ga kanipistondi. Ippati move chala healthy base nundi breakout la undi, late chase la kaadu.`
      }
    },
    {
      chapter: 2,
      title: "Momentum",
      titleTe: "మోమెంటమ్",
      icon: "Zap",
      score: bullScore - 4,
      text: {
        en: `Momentum is leaning positive with RSI holding in the power zone and MACD histogram expanding. This tells us buyers are still pressing, but only clean pullbacks should be chased.`,
        te: `RSI power zone lo undi, MACD histogram kuda expand avutondi kabatti momentum positive side lo undi. Buyers pressure chupistunnaru, kani clean pullback vachinappude entry better.`
      }
    },
    {
      chapter: 3,
      title: "Volume",
      titleTe: "వాల్యూం",
      icon: "BarChart3",
      score: bullScore - 8,
      text: {
        en: `Volume behaviour supports the move instead of contradicting it. That matters because this platform only upgrades conviction when price and participation travel together.`,
        te: `Volume move ni support chestondi, oppose cheyatledu. Price mariyu participation rendu kalisi vasthe matrame mana conviction increase chestam.`
      }
    },
    {
      chapter: 4,
      title: "Structure",
      titleTe: "స్ట్రక్చర్",
      icon: "Blocks",
      score: bullScore - 6,
      text: {
        en: `The active setup is tagged as ${signalLabel}, which means the pattern quality is already filtered by structure, risk-reward, and market regime. A failure back under the signal zone would weaken the thesis fast.`,
        te: `Current setup ${signalLabel} ga tag ayindi ante structure, risk-reward, market regime filters pass ayyi untayi. Signal zone kinda strong ga slip ayite thesis fast ga weak avutundi.`
      }
    },
    {
      chapter: 5,
      title: "Volatility",
      titleTe: "వోలాటిలిటీ",
      icon: "Activity",
      score: bullScore - 7,
      text: {
        en: `Volatility is controlled enough for a trend trade. That lowers noise, but it also means the stock should not spend too many candles under entry if the setup is genuine.`,
        te: `Trend trade ki saripoye controlled volatility undi. Noise takkuva untundi, kani setup nijam ayite entry kinda ekkuva candles waste avvakudadhu.`
      }
    },
    {
      chapter: 6,
      title: "Machine View",
      titleTe: "మిషన్ వ్యూ",
      icon: "Cpu",
      score: bullScore - 3,
      text: {
        en: `The ML layer agrees with the rule engine, so the stock is not just visually strong, it is statistically favored in the current regime. That does not remove risk, but it improves selection quality.`,
        te: `ML layer kuda rule engine ni support chestondi, kabatti idi chart lo bagundi ane matter matrame kaadu, current regime lo statistical edge kuda undi. Risk complete ga pothadu, kani stock selection quality improve avutundi.`
      }
    },
    {
      chapter: 7,
      title: "What To Watch",
      titleTe: "ఏమి గమనించాలి",
      icon: "Radar",
      score: bullScore - 10,
      text: {
        en: `Watch whether the stock holds the entry zone on intraday retests and whether sector leadership remains intact. If relative strength fades while Nifty stays firm, conviction should be reduced.`,
        te: `Intraday retest lo entry zone hold chestunda, sector leadership continue avutunda anedi watch cheyyali. Nifty strong ga unte kuda relative strength fade ayite conviction tagginchali.`
      }
    },
    {
      chapter: 8,
      title: "Verdict",
      titleTe: "తీర్పు",
      icon: "ShieldCheck",
      score: bullScore,
      text: {
        en: `${symbol} remains a monitored long candidate with disciplined execution only above the ideal zone. Treat it as a process trade: clean entry, predefined stop, staged exits.`,
        te: `${symbol} disciplined execution tho long candidate ga monitor cheyyachu, kani ideal zone paina matrame. Idi process trade laga handle cheyyali: clean entry, predefined stop, staged exits.`
      }
    }
  ];
}

function buildSignal(stock: {
  symbol: string;
  name: string;
  entry: number;
  stop: number;
  targets: [number, number, number];
  rr: number;
  score: number;
  screenerId: string;
  screenerName: string;
  status: SignalRecord["status"];
  verdict: SignalRecord["verdict"];
  snippetEn: string;
  snippetTe: string;
}): SignalRecord {
  return {
    id: `SIG-${stock.symbol}`,
    symbol: stock.symbol,
    company: stock.name,
    direction: "BULL",
    screenerId: stock.screenerId,
    screenerName: stock.screenerName,
    verdict: stock.verdict,
    status: stock.status,
    confluenceScore: stock.score,
    entry: stock.entry,
    stop: stock.stop,
    targets: stock.targets,
    rr: stock.rr,
    createdAt: "2026-03-13T09:45:00+05:30",
    snippet: {
      en: stock.snippetEn,
      te: stock.snippetTe
    }
  };
}

const stocks: StockRecord[] = [
  {
    instrumentId: 3045,
    symbol: "RELIANCE",
    name: "Reliance Industries",
    sector: "Energy",
    marketCapCr: 1940000,
    price: 2978.45,
    changePct: 1.84,
    bullScore: 86,
    bearScore: 18,
    trend: "Bullish",
    signalStatus: "ACTIVE",
    signalLabel: "SCR10 Perfect Storm",
    confluenceScore: 88,
    setup: {
      entry: 2968,
      zoneLow: 2954,
      zoneHigh: 2976,
      stop: 2898,
      targets: [3015, 3058, 3110],
      rr: 2.3,
      validUntil: "2026-03-13 15:20 IST"
    },
    whatToWatch: [
      "Hold above the 15M VWAP reclaim area.",
      "Energy basket breadth should remain positive.",
      "Volume ratio above 1.5x keeps the thesis intact."
    ],
    supportResistance: [2924, 2955, 3012, 3059],
    indicators: {
      close: 2978.45,
      ema9: 2954.12,
      ema20: 2938.41,
      ema50: 2896.84,
      ema200: 2748.55,
      rsi14: 63.4,
      rsi7: 68.1,
      macdHistogram: 2.41,
      volumeRatio: 2.2,
      obvSlope5: 1.7,
      cmf20: 0.18,
      adx: 27.6,
      plusDi: 31.4,
      minusDi: 16.2,
      bbSqueeze: false,
      supertrendDirection: 1,
      vwapDistancePct: 0.64,
      mtfAlignment: 8,
      confluenceScore: 88
    },
    scoreboard: {
      "1M": { bull: 69, bear: 26, trend: "bullish" },
      "5M": { bull: 73, bear: 24, trend: "bullish" },
      "15M": { bull: 79, bear: 18, trend: "bullish" },
      "1H": { bull: 84, bear: 16, trend: "bullish" },
      "4H": { bull: 81, bear: 19, trend: "bullish" },
      "1D": { bull: 88, bear: 14, trend: "bullish" }
    },
    story: buildStory("RELIANCE", "Energy", 86, "SCR10 Perfect Storm")
  },
  {
    instrumentId: 1594,
    symbol: "TCS",
    name: "Tata Consultancy Services",
    sector: "IT",
    marketCapCr: 1462000,
    price: 4244.15,
    changePct: 1.16,
    bullScore: 81,
    bearScore: 22,
    trend: "Bullish",
    signalStatus: "WATCHLIST",
    signalLabel: "SCR05 BB Squeeze",
    confluenceScore: 79,
    setup: {
      entry: 4252,
      zoneLow: 4218,
      zoneHigh: 4258,
      stop: 4162,
      targets: [4318, 4388, 4456],
      rr: 2.1,
      validUntil: "2026-03-13 15:20 IST"
    },
    whatToWatch: [
      "Needs a decisive close above squeeze range.",
      "IT index must stay leader for confirmation.",
      "Watch if 1H RSI remains above 55."
    ],
    supportResistance: [4180, 4225, 4308, 4385],
    indicators: {
      close: 4244.15,
      ema9: 4228.36,
      ema20: 4205.19,
      ema50: 4148.74,
      ema200: 3986.42,
      rsi14: 60.8,
      rsi7: 63.2,
      macdHistogram: 1.42,
      volumeRatio: 1.6,
      obvSlope5: 1.2,
      cmf20: 0.11,
      adx: 22.8,
      plusDi: 26.2,
      minusDi: 17.7,
      bbSqueeze: true,
      supertrendDirection: 1,
      vwapDistancePct: 0.22,
      mtfAlignment: 6,
      confluenceScore: 79
    },
    scoreboard: {
      "1M": { bull: 61, bear: 31, trend: "bullish" },
      "5M": { bull: 65, bear: 29, trend: "bullish" },
      "15M": { bull: 71, bear: 26, trend: "bullish" },
      "1H": { bull: 77, bear: 22, trend: "bullish" },
      "4H": { bull: 75, bear: 24, trend: "bullish" },
      "1D": { bull: 84, bear: 19, trend: "bullish" }
    },
    story: buildStory("TCS", "IT", 81, "SCR05 BB Squeeze")
  },
  {
    instrumentId: 4963,
    symbol: "HDFCBANK",
    name: "HDFC Bank",
    sector: "Banking",
    marketCapCr: 1248000,
    price: 1684.8,
    changePct: 0.82,
    bullScore: 78,
    bearScore: 28,
    trend: "Bullish",
    signalStatus: "ACTIVE",
    signalLabel: "SCR14 HHHL",
    confluenceScore: 76,
    setup: {
      entry: 1680,
      zoneLow: 1668,
      zoneHigh: 1684,
      stop: 1641,
      targets: [1704, 1726, 1752],
      rr: 2.2,
      validUntil: "2026-03-13 15:20 IST"
    },
    whatToWatch: [
      "Banks should keep relative strength over Nifty.",
      "Price should not close below yesterday's low.",
      "ADX expansion above 25 confirms continuation."
    ],
    supportResistance: [1650, 1672, 1708, 1735],
    indicators: {
      close: 1684.8,
      ema9: 1678.2,
      ema20: 1669.7,
      ema50: 1645.4,
      ema200: 1588.2,
      rsi14: 58.9,
      rsi7: 61.1,
      macdHistogram: 0.91,
      volumeRatio: 1.4,
      obvSlope5: 0.9,
      cmf20: 0.07,
      adx: 24.2,
      plusDi: 28.1,
      minusDi: 18.9,
      bbSqueeze: false,
      supertrendDirection: 1,
      vwapDistancePct: 0.33,
      mtfAlignment: 6,
      confluenceScore: 76
    },
    scoreboard: {
      "1M": { bull: 58, bear: 35, trend: "bullish" },
      "5M": { bull: 63, bear: 31, trend: "bullish" },
      "15M": { bull: 69, bear: 27, trend: "bullish" },
      "1H": { bull: 74, bear: 24, trend: "bullish" },
      "4H": { bull: 76, bear: 22, trend: "bullish" },
      "1D": { bull: 80, bear: 20, trend: "bullish" }
    },
    story: buildStory("HDFCBANK", "Banking", 78, "SCR14 HHHL")
  },
  {
    instrumentId: 9102,
    symbol: "SUNPHARMA",
    name: "Sun Pharmaceutical",
    sector: "Pharma",
    marketCapCr: 438200,
    price: 1812.25,
    changePct: -0.34,
    bullScore: 66,
    bearScore: 37,
    trend: "Neutral",
    signalStatus: "WATCHLIST",
    signalLabel: "SCR11 Demand Zone",
    confluenceScore: 68,
    setup: {
      entry: 1816,
      zoneLow: 1798,
      zoneHigh: 1818,
      stop: 1762,
      targets: [1845, 1874, 1905],
      rr: 1.9,
      validUntil: "2026-03-13 15:20 IST"
    },
    whatToWatch: [
      "Needs bounce candle confirmation at demand zone.",
      "Pharma breadth should improve intraday.",
      "Low-volume drift invalidates the setup."
    ],
    supportResistance: [1786, 1802, 1844, 1882],
    indicators: {
      close: 1812.25,
      ema9: 1815.66,
      ema20: 1807.41,
      ema50: 1782.12,
      ema200: 1714.33,
      rsi14: 51.3,
      rsi7: 48.8,
      macdHistogram: 0.22,
      volumeRatio: 0.96,
      obvSlope5: 0.2,
      cmf20: 0.03,
      adx: 19.4,
      plusDi: 21.1,
      minusDi: 18.4,
      bbSqueeze: true,
      supertrendDirection: 1,
      vwapDistancePct: -0.08,
      mtfAlignment: 4,
      confluenceScore: 68
    },
    scoreboard: {
      "1M": { bull: 48, bear: 44, trend: "neutral" },
      "5M": { bull: 53, bear: 38, trend: "neutral" },
      "15M": { bull: 58, bear: 35, trend: "bullish" },
      "1H": { bull: 62, bear: 33, trend: "bullish" },
      "4H": { bull: 66, bear: 30, trend: "bullish" },
      "1D": { bull: 70, bear: 28, trend: "bullish" }
    },
    story: buildStory("SUNPHARMA", "Pharma", 66, "SCR11 Demand Zone")
  },
  {
    instrumentId: 2210,
    symbol: "TATAMOTORS",
    name: "Tata Motors",
    sector: "Auto",
    marketCapCr: 331400,
    price: 946.6,
    changePct: 2.94,
    bullScore: 84,
    bearScore: 16,
    trend: "Bullish",
    signalStatus: "T1_HIT",
    signalLabel: "SCR01 52W Breakout",
    confluenceScore: 85,
    setup: {
      entry: 938,
      zoneLow: 930,
      zoneHigh: 941,
      stop: 912,
      targets: [952, 968, 988],
      rr: 2.5,
      validUntil: "2026-03-13 15:20 IST"
    },
    whatToWatch: [
      "Sustain above the breakout shelf near 938.",
      "Auto index rotation should remain strong.",
      "If price spikes without volume follow-through, trim conviction."
    ],
    supportResistance: [920, 938, 955, 981],
    indicators: {
      close: 946.6,
      ema9: 936.1,
      ema20: 922.4,
      ema50: 898.8,
      ema200: 826.5,
      rsi14: 67.2,
      rsi7: 72.8,
      macdHistogram: 3.06,
      volumeRatio: 2.9,
      obvSlope5: 2.3,
      cmf20: 0.21,
      adx: 31.8,
      plusDi: 35.5,
      minusDi: 13.4,
      bbSqueeze: false,
      supertrendDirection: 1,
      vwapDistancePct: 1.05,
      mtfAlignment: 8,
      confluenceScore: 85
    },
    scoreboard: {
      "1M": { bull: 73, bear: 22, trend: "bullish" },
      "5M": { bull: 77, bear: 19, trend: "bullish" },
      "15M": { bull: 82, bear: 16, trend: "bullish" },
      "1H": { bull: 85, bear: 15, trend: "bullish" },
      "4H": { bull: 83, bear: 18, trend: "bullish" },
      "1D": { bull: 87, bear: 13, trend: "bullish" }
    },
    story: buildStory("TATAMOTORS", "Auto", 84, "SCR01 52W Breakout")
  },
  {
    instrumentId: 7718,
    symbol: "INFY",
    name: "Infosys",
    sector: "IT",
    marketCapCr: 712600,
    price: 1668.35,
    changePct: -1.12,
    bullScore: 34,
    bearScore: 71,
    trend: "Bearish",
    signalStatus: "NONE",
    signalLabel: "No active signal",
    confluenceScore: 29,
    setup: {
      entry: 0,
      zoneLow: 0,
      zoneHigh: 0,
      stop: 0,
      targets: [0, 0, 0],
      rr: 0,
      validUntil: "No setup"
    },
    whatToWatch: [
      "Needs reclaim of 20 EMA before it belongs on the long list.",
      "IT sector breadth has to stabilize first.",
      "Below 1650, sellers keep control."
    ],
    supportResistance: [1650, 1682, 1710, 1742],
    indicators: {
      close: 1668.35,
      ema9: 1675.4,
      ema20: 1684.1,
      ema50: 1701.7,
      ema200: 1628.8,
      rsi14: 41.9,
      rsi7: 36.5,
      macdHistogram: -1.86,
      volumeRatio: 1.1,
      obvSlope5: -0.8,
      cmf20: -0.06,
      adx: 23.2,
      plusDi: 16.9,
      minusDi: 29.4,
      bbSqueeze: false,
      supertrendDirection: -1,
      vwapDistancePct: -0.74,
      mtfAlignment: 2,
      confluenceScore: 29
    },
    scoreboard: {
      "1M": { bull: 28, bear: 61, trend: "bearish" },
      "5M": { bull: 26, bear: 64, trend: "bearish" },
      "15M": { bull: 30, bear: 67, trend: "bearish" },
      "1H": { bull: 34, bear: 71, trend: "bearish" },
      "4H": { bull: 38, bear: 66, trend: "bearish" },
      "1D": { bull: 32, bear: 74, trend: "bearish" }
    },
    story: buildStory("INFY", "IT", 34, "No active signal")
  }
];

stocks[0].signal = buildSignal({
  symbol: "RELIANCE",
  name: "Reliance Industries",
  entry: 2968,
  stop: 2898,
  targets: [3015, 3058, 3110],
  rr: 2.3,
  score: 88,
  screenerId: "SCR10",
  screenerName: "Perfect Storm",
  status: "ACTIVE",
  verdict: "APPROVE",
  snippetEn:
    "Alignment is clean across daily and hourly structure, and the signal still has room before exhaustion.",
  snippetTe:
    "Daily mariyu hourly alignment clean ga undi, inka move ki space undi kabatti signal strong ga undi."
});
stocks[1].signal = buildSignal({
  symbol: "TCS",
  name: "Tata Consultancy Services",
  entry: 4252,
  stop: 4162,
  targets: [4318, 4388, 4456],
  rr: 2.1,
  score: 79,
  screenerId: "SCR05",
  screenerName: "BB Squeeze",
  status: "WATCHLIST",
  verdict: "WATCHLIST",
  snippetEn:
    "Squeeze quality is good, but the engine wants a stronger expansion candle before full approval.",
  snippetTe:
    "Squeeze quality bagundi, kani full approval kosam inka stronger expansion candle kavali."
});
stocks[2].signal = buildSignal({
  symbol: "HDFCBANK",
  name: "HDFC Bank",
  entry: 1680,
  stop: 1641,
  targets: [1704, 1726, 1752],
  rr: 2.2,
  score: 76,
  screenerId: "SCR14",
  screenerName: "HHHL Continuation",
  status: "ACTIVE",
  verdict: "APPROVE",
  snippetEn:
    "This is a cleaner continuation setup than a breakout chase, which helps on a higher beta banking tape.",
  snippetTe:
    "Idi breakout chase kanna cleaner continuation setup, especially banking tape lo idi better ga panicheyyachu."
});
stocks[3].signal = buildSignal({
  symbol: "SUNPHARMA",
  name: "Sun Pharmaceutical",
  entry: 1816,
  stop: 1762,
  targets: [1845, 1874, 1905],
  rr: 1.9,
  score: 68,
  screenerId: "SCR11",
  screenerName: "Demand Zone",
  status: "WATCHLIST",
  verdict: "WATCHLIST",
  snippetEn:
    "The location is decent, but there is not enough urgency in participation yet.",
  snippetTe:
    "Location decent ga undi, kani participation lo urgency inka sariga ledu."
});
stocks[4].signal = buildSignal({
  symbol: "TATAMOTORS",
  name: "Tata Motors",
  entry: 938,
  stop: 912,
  targets: [952, 968, 988],
  rr: 2.5,
  score: 85,
  screenerId: "SCR01",
  screenerName: "52W Breakout",
  status: "T1_HIT",
  verdict: "APPROVE",
  snippetEn:
    "Breakout quality is holding after first target, which keeps the trade in staged-exit mode instead of full booking.",
  snippetTe:
    "First target taruvata kuda breakout quality hold avtundi, anduke staged exits continue cheyyachu."
});

const marketContext: MarketContext = {
  regime: "BULL_TRENDING",
  nifty: {
    price: 24186.4,
    changePct: 0.92,
    outlook: "Broad participation with leadership from auto, energy, and private banks."
  },
  vix: {
    value: 13.7,
    trend: "Cooling"
  },
  fii: {
    flowCr: 742,
    status: "Net buyers for the third straight session"
  },
  topSectors: ["Auto", "Energy", "Private Banks"]
};

const userProfile: UserProfile = {
  name: "Akhil Reddy",
  email: "akhil@neuraltrade.local",
  language: "te",
  plan: "PRO",
  watchlist: ["RELIANCE", "TCS", "HDFCBANK", "SUNPHARMA"],
  quietHours: "10:00 PM - 9:00 AM",
  alertTypes: ["Signal Alert", "Morning Briefing", "EOD Summary"],
  telegramConnected: true
};

const plans: SubscriptionPlan[] = [
  {
    id: "BASIC",
    price: "Rs499/mo",
    highlight: "Rule-based stories and daily briefing for focused retail users.",
    features: [
      "Up to 5 signals per day",
      "English story engine",
      "Morning briefing alerts"
    ]
  },
  {
    id: "PRO",
    price: "Rs999/mo",
    highlight: "Full scoreboard, Telugu stories, and validated live signal feed.",
    features: [
      "All live signals",
      "Telugu and English stories",
      "On-demand deep analysis with daily cap"
    ]
  },
  {
    id: "ELITE",
    price: "Rs1999/mo",
    highlight: "Priority delivery, unlimited deep analysis, and sector reports.",
    features: [
      "Unlimited deep analysis",
      "Priority alert delivery",
      "Weekly sector intelligence"
    ]
  }
];

function clone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

export const demoData = {
  marketContext,
  userProfile,
  plans,
  stocks,
  morningBriefing: {
    en: "Momentum breadth is healthy and the system is favoring continuation setups over aggressive mean reversion. Perfect Storm and 52-week breakout names remain the best quality basket today.",
    te: "Momentum breadth healthy ga undi, kabatti system ippudu continuation setups ni ekkuva prefer chestondi. Perfect Storm mariyu 52-week breakout names ivvala best quality basket lo unnayi."
  }
};

export function getStocks(): StockRecord[] {
  return clone(demoData.stocks);
}

export function getStock(symbol: string): StockRecord | undefined {
  return clone(
    demoData.stocks.find((stock) => stock.symbol.toLowerCase() === symbol.toLowerCase())
  );
}

export function getSignals(): SignalRecord[] {
  return clone(
    demoData.stocks
      .map((stock) => stock.signal)
      .filter((signal): signal is SignalRecord => Boolean(signal))
  );
}

export function getLocalizedText(
  text: Record<Language, string>,
  language: Language
): string {
  return text[language];
}
