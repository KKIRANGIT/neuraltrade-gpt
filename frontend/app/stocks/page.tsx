import Link from "next/link";
import { AppShell } from "@/components/layout/AppShell";
import { PageHeader } from "@/components/layout/PageHeader";
import { getAllStocks, getLanguageFromSearchParam } from "@/lib/api/stocksApi";

export default async function StocksPage({
  searchParams
}: {
  searchParams?: Record<string, string | string[] | undefined>;
}) {
  const params = searchParams ?? {};
  const language = getLanguageFromSearchParam(
    Array.isArray(params.lang) ? params.lang[0] : params.lang
  );
  const sector = Array.isArray(params.sector) ? params.sector[0] : params.sector;
  const status = Array.isArray(params.status) ? params.status[0] : params.status;
  const query = Array.isArray(params.q) ? params.q[0] : params.q;
  const stocks = await getAllStocks({ sector, status, query });

  return (
    <AppShell activePath="/stocks" language={language}>
      <div className="space-y-6">
        <PageHeader
          eyebrow="Universe"
          title="Sortable stock board with score, trend, and signal state"
          description="The stock board is the operator view of the platform. In production it would stream live scoreboard refreshes every minute; this implementation ships with a seeded universe and filtering surface."
        />
        <section className="rounded-[30px] border border-white/70 bg-white/90 p-6 shadow-panel">
          <div className="flex flex-wrap gap-2 text-sm">
            {["ALL", "Energy", "IT", "Banking", "Pharma", "Auto"].map((value) => (
              <Link
                key={value}
                href={`/stocks?lang=${language}&sector=${value}`}
                className={`rounded-full px-4 py-2 ${sector === value || (!sector && value === "ALL") ? "bg-ink text-white" : "bg-ink/5 text-ink"}`}
              >
                {value}
              </Link>
            ))}
            {["ALL", "ACTIVE", "WATCHLIST", "T1_HIT", "NONE"].map((value) => (
              <Link
                key={value}
                href={`/stocks?lang=${language}&status=${value}`}
                className={`rounded-full px-4 py-2 ${status === value || (!status && value === "ALL") ? "bg-pine text-white" : "bg-pine/10 text-pine"}`}
              >
                {value.replaceAll("_", " ")}
              </Link>
            ))}
          </div>
          <div className="mt-6 overflow-x-auto">
            <table className="min-w-full text-left text-sm">
              <thead>
                <tr className="border-b border-ink/10 text-ink/55">
                  <th className="pb-3">Symbol</th>
                  <th className="pb-3">Company</th>
                  <th className="pb-3">Sector</th>
                  <th className="pb-3">Bull</th>
                  <th className="pb-3">Bear</th>
                  <th className="pb-3">Signal</th>
                  <th className="pb-3">Trend</th>
                </tr>
              </thead>
              <tbody>
                {stocks.map((stock) => (
                  <tr key={stock.symbol} className="border-b border-ink/5">
                    <td className="py-4 font-semibold">
                      <Link href={`/stocks/${stock.symbol}?lang=${language}`}>{stock.symbol}</Link>
                    </td>
                    <td className="py-4 text-ink/70">{stock.name}</td>
                    <td className="py-4 text-ink/70">{stock.sector}</td>
                    <td className="py-4 font-semibold text-pine">{stock.bullScore}</td>
                    <td className="py-4 font-semibold text-ember">{stock.bearScore}</td>
                    <td className="py-4 text-ink/70">{stock.signalStatus.replaceAll("_", " ")}</td>
                    <td className="py-4 text-ink/70">{stock.trend}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </AppShell>
  );
}
