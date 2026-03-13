import Link from "next/link";
import { AppShell } from "@/components/layout/AppShell";
import { PageHeader } from "@/components/layout/PageHeader";
import { SignalCard } from "@/components/signals/SignalCard";
import { StoryCard } from "@/components/story/StoryCard";
import {
  getLanguageFromSearchParam,
  getPlans,
  getTopBullStocks
} from "@/lib/api/stocksApi";

export default async function LandingPage({
  searchParams
}: {
  searchParams?: Record<string, string | string[] | undefined>;
}) {
  const params = searchParams ?? {};
  const language = getLanguageFromSearchParam(
    Array.isArray(params.lang) ? params.lang[0] : params.lang
  );
  const [topStocks, plans] = await Promise.all([getTopBullStocks(1), getPlans()]);
  const heroStock = topStocks[0];

  return (
    <AppShell activePath="/" language={language}>
      <div className="space-y-6">
        <PageHeader
          eyebrow="Platform"
          title="Signal generation, stock storytelling, and trader context in one surface"
          description="This reference build implements the documented NeuralTrade experience with seeded Indian market data, multi-timeframe scoreboards, validated signals, Telugu and English stories, and subscription-aware product surfaces."
          action={
            <Link
              href={`/dashboard?lang=${language}`}
              className="inline-flex rounded-full bg-ink px-5 py-3 text-sm font-semibold text-white"
            >
              Open dashboard
            </Link>
          }
        />

        <section className="grid gap-6 lg:grid-cols-[1.25fr_0.75fr]">
          <div className="rounded-[32px] border border-white/70 bg-white/90 p-8 shadow-panel">
            <p className="text-xs uppercase tracking-[0.3em] text-pine">Why this platform exists</p>
            <h2 className="mt-3 font-[var(--font-display)] text-4xl font-semibold text-ink">
              Professional analysis without forcing users to decode charts
            </h2>
            <div className="mt-6 grid gap-4 sm:grid-cols-3">
              {[
                ["500-stock universe", "Ranked by bull, bear, and confluence signals."],
                ["Telugu + English stories", "Eight concise chapters that explain what the engine sees."],
                ["Validated alerts", "Signals include Claude verdicts, trade ladders, and lifecycle states."]
              ].map(([title, copy]) => (
                <div key={title} className="rounded-3xl bg-ink/5 p-4">
                  <h3 className="font-semibold text-ink">{title}</h3>
                  <p className="mt-2 text-sm leading-6 text-ink/70">{copy}</p>
                </div>
              ))}
            </div>
          </div>

          <div className="rounded-[32px] border border-pine/20 bg-gradient-to-br from-pine/10 via-white to-copper/10 p-8 shadow-panel">
            <p className="text-xs uppercase tracking-[0.3em] text-copper">Live sample</p>
            <h2 className="mt-3 text-3xl font-semibold text-ink">{heroStock.symbol}</h2>
            <p className="mt-2 text-sm leading-7 text-ink/70">
              Bull score {heroStock.bullScore}/100, confluence {heroStock.confluenceScore}/100,
              active setup {heroStock.signalLabel}.
            </p>
            <div className="mt-6 space-y-3">
              {heroStock.whatToWatch.map((item) => (
                <div key={item} className="rounded-2xl bg-white/85 px-4 py-3 text-sm text-ink/75">
                  {item}
                </div>
              ))}
            </div>
          </div>
        </section>

        {heroStock.signal ? <SignalCard signal={heroStock.signal} language={language} /> : null}

        <section className="grid gap-6 lg:grid-cols-2">
          <div className="space-y-4">
            <p className="text-xs uppercase tracking-[0.3em] text-copper">Story demo</p>
            {heroStock.story.slice(0, 2).map((chapter) => (
              <StoryCard key={chapter.chapter} chapter={chapter} language={language} />
            ))}
          </div>
          <div className="rounded-[30px] border border-white/70 bg-white/90 p-6 shadow-panel">
            <p className="text-xs uppercase tracking-[0.3em] text-copper">Pricing</p>
            <div className="mt-4 grid gap-4">
              {plans.map((plan) => (
                <div key={plan.id} className="rounded-3xl border border-ink/10 p-5">
                  <div className="flex items-baseline justify-between">
                    <h3 className="text-xl font-semibold text-ink">{plan.id}</h3>
                    <span className="text-sm font-semibold text-pine">{plan.price}</span>
                  </div>
                  <p className="mt-2 text-sm leading-6 text-ink/70">{plan.highlight}</p>
                  <ul className="mt-4 space-y-2 text-sm text-ink/75">
                    {plan.features.map((feature) => (
                      <li key={feature}>{feature}</li>
                    ))}
                  </ul>
                </div>
              ))}
            </div>
          </div>
        </section>
      </div>
    </AppShell>
  );
}
