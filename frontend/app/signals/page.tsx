import { AppShell } from "@/components/layout/AppShell";
import { PageHeader } from "@/components/layout/PageHeader";
import { SignalCard } from "@/components/signals/SignalCard";
import { getLanguageFromSearchParam, getSignalFeed } from "@/lib/api/stocksApi";

export default async function SignalsPage({
  searchParams
}: {
  searchParams?: Record<string, string | string[] | undefined>;
}) {
  const params = searchParams ?? {};
  const language = getLanguageFromSearchParam(
    Array.isArray(params.lang) ? params.lang[0] : params.lang
  );
  const status = Array.isArray(params.status) ? params.status[0] : params.status;
  const signals = await getSignalFeed(status);

  return (
    <AppShell activePath="/signals" language={language}>
      <div className="space-y-6">
        <PageHeader
          eyebrow="Signals"
          title="Validated signal feed with trade ladders and status tracking"
          description="Each card represents the output after screening, scoring, ML context, and AI validation. In the live system this feed would update from Kafka-driven lifecycle events."
        />
        <div className="flex flex-wrap gap-2 text-sm">
          {["ALL", "ACTIVE", "WATCHLIST", "T1_HIT"].map((value) => (
            <a
              key={value}
              href={`/signals?lang=${language}&status=${value}`}
              className={`rounded-full px-4 py-2 ${status === value || (!status && value === "ALL") ? "bg-ink text-white" : "bg-ink/5 text-ink"}`}
            >
              {value.replaceAll("_", " ")}
            </a>
          ))}
        </div>
        <div className="space-y-4">
          {signals.map((signal) => (
            <SignalCard key={signal.id} signal={signal} language={language} />
          ))}
        </div>
      </div>
    </AppShell>
  );
}
