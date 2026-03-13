import Link from "next/link";
import { AppShell } from "@/components/layout/AppShell";
import { PageHeader } from "@/components/layout/PageHeader";
import { MarketContextBar } from "@/components/market/MarketContextBar";
import { SignalCard } from "@/components/signals/SignalCard";
import {
  getLanguageFromSearchParam,
  getMarketContext,
  getMorningBriefing,
  getSignalFeed,
  getTopBearStocks,
  getTopBullStocks
} from "@/lib/api/stocksApi";

export default async function DashboardPage({
  searchParams
}: {
  searchParams?: Record<string, string | string[] | undefined>;
}) {
  const params = searchParams ?? {};
  const language = getLanguageFromSearchParam(
    Array.isArray(params.lang) ? params.lang[0] : params.lang
  );
  const [context, briefing, bullLeaders, bearLeaders, signals] = await Promise.all([
    getMarketContext(),
    getMorningBriefing(language),
    getTopBullStocks(3),
    getTopBearStocks(3),
    getSignalFeed()
  ]);

  return (
    <AppShell activePath="/dashboard" language={language}>
      <div className="space-y-6">
        <PageHeader
          eyebrow="Dashboard"
          title="Live regime, score leaders, and validated signals"
          description="The dashboard combines the product's morning market briefing, current regime, top-ranked stocks, and the trade feed that would normally be driven by the Rust engines plus AI validation."
          action={
            <Link
              href={`/signals?lang=${language}`}
              className="inline-flex rounded-full bg-ink px-5 py-3 text-sm font-semibold text-white"
            >
              Open signal feed
            </Link>
          }
        />
        <MarketContextBar context={context} />

        <section className="grid gap-6 lg:grid-cols-[1.15fr_0.85fr]">
          <div className="rounded-[30px] border border-white/70 bg-white/90 p-6 shadow-panel">
            <p className="text-xs uppercase tracking-[0.3em] text-copper">Morning briefing</p>
            <p className="mt-4 text-lg leading-8 text-ink/80">{briefing}</p>
          </div>
          <div className="grid gap-4">
            <div className="rounded-[30px] border border-white/70 bg-white/90 p-6 shadow-panel">
              <p className="text-xs uppercase tracking-[0.3em] text-pine">Top bulls</p>
              <div className="mt-4 space-y-3">
                {bullLeaders.map((stock) => (
                  <Link
                    key={stock.symbol}
                    href={`/stocks/${stock.symbol}?lang=${language}`}
                    className="flex items-center justify-between rounded-2xl bg-pine/5 px-4 py-3"
                  >
                    <span>
                      <span className="block font-semibold text-ink">{stock.symbol}</span>
                      <span className="text-sm text-ink/60">{stock.signalLabel}</span>
                    </span>
                    <span className="text-lg font-bold text-pine">{stock.bullScore}</span>
                  </Link>
                ))}
              </div>
            </div>
            <div className="rounded-[30px] border border-white/70 bg-white/90 p-6 shadow-panel">
              <p className="text-xs uppercase tracking-[0.3em] text-ember">Top bears</p>
              <div className="mt-4 space-y-3">
                {bearLeaders.map((stock) => (
                  <Link
                    key={stock.symbol}
                    href={`/stocks/${stock.symbol}?lang=${language}`}
                    className="flex items-center justify-between rounded-2xl bg-ember/5 px-4 py-3"
                  >
                    <span>
                      <span className="block font-semibold text-ink">{stock.symbol}</span>
                      <span className="text-sm text-ink/60">{stock.trend}</span>
                    </span>
                    <span className="text-lg font-bold text-ember">{stock.bearScore}</span>
                  </Link>
                ))}
              </div>
            </div>
          </div>
        </section>

        <section className="space-y-4">
          <p className="text-xs uppercase tracking-[0.3em] text-copper">Today's signal feed</p>
          {signals.map((signal) => (
            <SignalCard key={signal.id} signal={signal} language={language} />
          ))}
        </section>
      </div>
    </AppShell>
  );
}
