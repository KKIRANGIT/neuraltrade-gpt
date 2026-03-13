import { type Language, type SignalRecord } from "@/lib/types";

function statusTone(status: SignalRecord["status"]) {
  if (status === "ACTIVE") {
    return "bg-pine/15 text-pine";
  }
  if (status.includes("HIT")) {
    return "bg-copper/15 text-copper";
  }
  if (status === "WATCHLIST") {
    return "bg-sky-100 text-sky-700";
  }
  return "bg-ember/15 text-ember";
}

export function SignalCard({
  signal,
  language
}: {
  signal: SignalRecord;
  language: Language;
}) {
  return (
    <article className="rounded-[28px] border border-white/70 bg-white/90 p-6 shadow-panel">
      <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
        <div className="space-y-3">
          <div className="flex flex-wrap items-center gap-2">
            <span className="rounded-full bg-ink px-3 py-1 text-xs font-semibold uppercase tracking-[0.25em] text-white">
              {signal.symbol}
            </span>
            <span className={`rounded-full px-3 py-1 text-xs font-semibold ${statusTone(signal.status)}`}>
              {signal.status.replaceAll("_", " ")}
            </span>
            <span className="rounded-full bg-amber-100 px-3 py-1 text-xs font-semibold text-amber-700">
              Claude {signal.verdict}
            </span>
          </div>
          <div>
            <h3 className="text-xl font-semibold text-ink">{signal.company}</h3>
            <p className="text-sm text-ink/60">
              {signal.screenerId} · {signal.screenerName} · Score {signal.confluenceScore}/100
            </p>
          </div>
          <p className="max-w-2xl text-sm leading-7 text-ink/75">{signal.snippet[language]}</p>
        </div>
        <div className="min-w-[260px] rounded-3xl bg-ink p-4 text-white">
          <p className="text-xs uppercase tracking-[0.25em] text-white/55">Trade ladder</p>
          <div className="mt-4 grid gap-2 text-sm">
            <div className="flex justify-between">
              <span>Entry</span>
              <span>Rs{signal.entry.toFixed(2)}</span>
            </div>
            <div className="flex justify-between">
              <span>Stop</span>
              <span>Rs{signal.stop.toFixed(2)}</span>
            </div>
            {signal.targets.map((target, index) => (
              <div key={target} className="flex justify-between">
                <span>T{index + 1}</span>
                <span>Rs{target.toFixed(2)}</span>
              </div>
            ))}
            <div className="mt-2 border-t border-white/10 pt-2 text-copper">
              <div className="flex justify-between">
                <span>R:R</span>
                <span>1:{signal.rr.toFixed(1)}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </article>
  );
}
