import { type MarketContext } from "@/lib/types";

function Pill({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-2xl bg-ink/5 px-4 py-3">
      <p className="text-xs uppercase tracking-[0.25em] text-ink/45">{label}</p>
      <p className="mt-1 text-sm font-semibold text-ink">{value}</p>
    </div>
  );
}

export function MarketContextBar({ context }: { context: MarketContext }) {
  return (
    <section className="grid gap-4 rounded-[28px] border border-pine/20 bg-gradient-to-r from-pine/10 via-white to-copper/10 p-5 shadow-panel lg:grid-cols-[1.6fr_repeat(4,minmax(0,1fr))]">
      <div>
        <p className="text-xs font-semibold uppercase tracking-[0.35em] text-pine">Market regime</p>
        <h3 className="mt-2 text-2xl font-bold text-ink">{context.regime.replaceAll("_", " ")}</h3>
        <p className="mt-2 max-w-xl text-sm leading-6 text-ink/70">{context.nifty.outlook}</p>
      </div>
      <Pill
        label="Nifty"
        value={`${context.nifty.price.toFixed(1)} (${context.nifty.changePct > 0 ? "+" : ""}${context.nifty.changePct.toFixed(2)}%)`}
      />
      <Pill label="India VIX" value={`${context.vix.value.toFixed(1)} · ${context.vix.trend}`} />
      <Pill label="FII" value={`${context.fii.status} · Rs${context.fii.flowCr}cr`} />
      <Pill label="Rotation" value={context.topSectors.join(" · ")} />
    </section>
  );
}
