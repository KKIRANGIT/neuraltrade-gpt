import { demoData, getSignals, getStock, getStocks } from "@/lib/data/demoData";
import {
  type Language,
  type MarketContext,
  type SignalRecord,
  type StockRecord,
  type SubscriptionPlan,
  type UserProfile
} from "@/lib/types";

const delay = (ms = 120) => new Promise((resolve) => setTimeout(resolve, ms));

export async function getAllStocks(filters?: {
  sector?: string;
  status?: string;
  query?: string;
}): Promise<StockRecord[]> {
  await delay();
  return getStocks().filter((stock) => {
    const sectorOk = !filters?.sector || filters.sector === "ALL" || stock.sector === filters.sector;
    const statusOk =
      !filters?.status || filters.status === "ALL" || stock.signalStatus === filters.status;
    const queryOk =
      !filters?.query ||
      stock.symbol.toLowerCase().includes(filters.query.toLowerCase()) ||
      stock.name.toLowerCase().includes(filters.query.toLowerCase());
    return sectorOk && statusOk && queryOk;
  });
}

export async function getTopBullStocks(limit = 3): Promise<StockRecord[]> {
  await delay();
  return getStocks()
    .sort((left, right) => right.bullScore - left.bullScore)
    .slice(0, limit);
}

export async function getTopBearStocks(limit = 3): Promise<StockRecord[]> {
  await delay();
  return getStocks()
    .sort((left, right) => right.bearScore - left.bearScore)
    .slice(0, limit);
}

export async function getStockDetail(symbol: string): Promise<StockRecord | undefined> {
  await delay();
  return getStock(symbol);
}

export async function getSignalFeed(status?: string): Promise<SignalRecord[]> {
  await delay();
  return getSignals().filter((signal) => !status || status === "ALL" || signal.status === status);
}

export async function getMarketContext(): Promise<MarketContext> {
  await delay(60);
  return structuredClone(demoData.marketContext);
}

export async function getMorningBriefing(language: Language): Promise<string> {
  await delay(60);
  return demoData.morningBriefing[language];
}

export async function getUserProfile(): Promise<UserProfile> {
  await delay(60);
  return structuredClone(demoData.userProfile);
}

export async function getPlans(): Promise<SubscriptionPlan[]> {
  await delay(60);
  return structuredClone(demoData.plans);
}

export function getLanguageFromSearchParam(value?: string): Language {
  return value === "te" ? "te" : "en";
}
