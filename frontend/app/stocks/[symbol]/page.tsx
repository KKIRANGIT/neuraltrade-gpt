import Link from "next/link";
import { notFound } from "next/navigation";
import { AppShell } from "@/components/layout/AppShell";
import { PageHeader } from "@/components/layout/PageHeader";
import { ScoreboardWidget } from "@/components/scoreboard/ScoreboardWidget";
import { StoryCard } from "@/components/story/StoryCard";
import { getLanguageFromSearchParam, getStockDetail } from "@/lib/api/stocksApi";

export default async function StockDetailPage({
  params,
  searchParams
}: {
  params: { symbol: string };
  searchParams?: Record<string, string | string[] | undefined>;
}) {
  const { symbol } = params;
  const resolvedSearchParams = searchParams;
  const language = getLanguageFromSearchParam(
    Array.isArray(resolvedSearchParams?.lang)
      ? resolvedSearchParams?.lang[0]
      : resolvedSearchParams?.lang
  );
  const stock = await getStockDetail(symbol);

  if (!stock) {
    notFound();
  }

  return (
    <AppShell activePath="/stocks" languagePath={`/stocks/${stock.symbol}`} language={language}>
      <div className="space-y-6">
        <PageHeader
          eyebrow={stock.sector}
          title={`${stock.symbol} · ${stock.name}`}
          description={`Price Rs${stock.price.toFixed(2)} (${stock.changePct > 0 ? "+" : ""}${stock.changePct.toFixed(2)}%), bull score ${stock.bullScore}, bear score ${stock.bearScore}, confluence ${stock.confluenceScore}.`}
          action={
            <Link
              href={`/signals?lang=${language}`}
              className="inline-flex rounded-full bg-ink px-5 py-3 text-sm font-semibold text-white"
            >
              View signal feed
            </Link>
          }
        />

        <section className="grid gap-6 xl:grid-cols-[0.9fr_1.15fr_0.95fr]">
          <ScoreboardWidget stock={stock} />

          <div className="space-y-4">
            {stock.story.map((chapter) => (
              <StoryCard key={chapter.chapter} chapter={chapter} language={language} />
            ))}
          </div>

          <div className="space-y-6">
            <section className="rounded-[28px] border border-white/70 bg-white/90 p-6 shadow-panel">
              <p className="text-xs uppercase tracking-[0.3em] text-copper">Trade setup</p>
              <div className="mt-4 grid gap-3 text-sm">
                <div className="flex justify-between">
                  <span>Ideal entry</span>
                  <span>Rs{stock.setup.entry.toFixed(2)}</span>
                </div>
                <div className="flex justify-between">
                  <span>Zone</span>
                  <span>
                    Rs{stock.setup.zoneLow.toFixed(2)} - Rs{stock.setup.zoneHigh.toFixed(2)}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span>Stop</span>
                  <span>Rs{stock.setup.stop.toFixed(2)}</span>
                </div>
                {stock.setup.targets.map((target, index) => (
                  <div key={target} className="flex justify-between">
                    <span>T{index + 1}</span>
                    <span>Rs{target.toFixed(2)}</span>
                  </div>
                ))}
                <div className="flex justify-between border-t border-ink/10 pt-3">
                  <span>Risk / Reward</span>
                  <span>1:{stock.setup.rr.toFixed(1)}</span>
                </div>
              </div>
            </section>

            <section className="rounded-[28px] border border-white/70 bg-white/90 p-6 shadow-panel">
              <p className="text-xs uppercase tracking-[0.3em] text-pine">Indicator snapshot</p>
              <div className="mt-4 grid grid-cols-2 gap-3 text-sm">
                <div className="rounded-2xl bg-ink/5 p-3">RSI 14: {stock.indicators.rsi14.toFixed(1)}</div>
                <div className="rounded-2xl bg-ink/5 p-3">RSI 7: {stock.indicators.rsi7.toFixed(1)}</div>
                <div className="rounded-2xl bg-ink/5 p-3">MACD Hist: {stock.indicators.macdHistogram.toFixed(2)}</div>
                <div className="rounded-2xl bg-ink/5 p-3">Volume Ratio: {stock.indicators.volumeRatio.toFixed(2)}x</div>
                <div className="rounded-2xl bg-ink/5 p-3">ADX: {stock.indicators.adx.toFixed(1)}</div>
                <div className="rounded-2xl bg-ink/5 p-3">CMF20: {stock.indicators.cmf20.toFixed(2)}</div>
                <div className="rounded-2xl bg-ink/5 p-3">
                  MTF Alignment: {stock.indicators.mtfAlignment}/8
                </div>
                <div className="rounded-2xl bg-ink/5 p-3">
                  VWAP Distance: {stock.indicators.vwapDistancePct.toFixed(2)}%
                </div>
              </div>
            </section>

            <section className="rounded-[28px] border border-white/70 bg-white/90 p-6 shadow-panel">
              <p className="text-xs uppercase tracking-[0.3em] text-copper">What to watch</p>
              <ul className="mt-4 space-y-3 text-sm leading-7 text-ink/75">
                {stock.whatToWatch.map((item) => (
                  <li key={item}>{item}</li>
                ))}
              </ul>
            </section>
          </div>
        </section>
      </div>
    </AppShell>
  );
}
