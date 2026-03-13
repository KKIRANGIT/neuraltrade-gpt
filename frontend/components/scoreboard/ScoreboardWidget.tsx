import { type StockRecord, type TimeframeKey } from "@/lib/types";

const orderedTimeframes: TimeframeKey[] = ["1M", "5M", "15M", "1H", "4H", "1D"];

export function ScoreboardWidget({ stock }: { stock: StockRecord }) {
  return (
    <section className="rounded-[28px] border border-white/70 bg-white/90 p-6 shadow-panel">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-xs uppercase tracking-[0.3em] text-copper">Scoreboard</p>
          <h3 className="mt-1 text-2xl font-semibold text-ink">{stock.symbol}</h3>
        </div>
        <div className="text-right">
          <p className="text-sm text-ink/55">Total bull score</p>
          <p className="text-4xl font-bold text-pine">{stock.bullScore}</p>
        </div>
      </div>
      <div className="mt-6 space-y-4">
        {orderedTimeframes.map((timeframe) => {
          const score = stock.scoreboard[timeframe];
          return (
            <div key={timeframe} className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="font-semibold text-ink">{timeframe}</span>
                <span className="text-ink/55">
                  Bull {score.bull} / Bear {score.bear}
                </span>
              </div>
              <div className="grid grid-cols-2 gap-2">
                <div className="h-3 overflow-hidden rounded-full bg-pine/10">
                  <div className="h-full rounded-full bg-pine" style={{ width: `${score.bull}%` }} />
                </div>
                <div className="h-3 overflow-hidden rounded-full bg-ember/10">
                  <div className="h-full rounded-full bg-ember" style={{ width: `${score.bear}%` }} />
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </section>
  );
}
