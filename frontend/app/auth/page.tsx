import Link from "next/link";
import { AppShell } from "@/components/layout/AppShell";
import { PageHeader } from "@/components/layout/PageHeader";
import { getLanguageFromSearchParam, getPlans } from "@/lib/api/stocksApi";

export default async function AuthPage({
  searchParams
}: {
  searchParams?: Record<string, string | string[] | undefined>;
}) {
  const params = searchParams ?? {};
  const language = getLanguageFromSearchParam(
    Array.isArray(params.lang) ? params.lang[0] : params.lang
  );
  const plans = await getPlans();

  return (
    <AppShell activePath="/auth" language={language}>
      <div className="space-y-6">
        <PageHeader
          eyebrow="Access"
          title="Sign in to the trading workspace"
          description="This page represents the authentication and plan handoff flow. It is wired for the documented product tiers and keeps the UI aligned with PRO and ELITE features."
        />
        <section className="grid gap-6 lg:grid-cols-[0.9fr_1.1fr]">
          <div className="rounded-[28px] border border-white/70 bg-white/90 p-6 shadow-panel">
            <p className="text-xs uppercase tracking-[0.3em] text-copper">Demo login</p>
            <div className="mt-5 space-y-4">
              <div>
                <label className="text-sm font-medium text-ink/70">Email</label>
                <div className="mt-2 rounded-2xl border border-ink/10 bg-mist px-4 py-3 text-sm">
                  akhil@neuraltrade.local
                </div>
              </div>
              <div>
                <label className="text-sm font-medium text-ink/70">Password</label>
                <div className="mt-2 rounded-2xl border border-ink/10 bg-mist px-4 py-3 text-sm">
                  neuraltrade-demo
                </div>
              </div>
              <Link
                href={`/dashboard?lang=${language}`}
                className="inline-flex rounded-full bg-ink px-5 py-3 text-sm font-semibold text-white"
              >
                Continue to dashboard
              </Link>
            </div>
          </div>
          <div className="rounded-[28px] border border-white/70 bg-white/90 p-6 shadow-panel">
            <p className="text-xs uppercase tracking-[0.3em] text-pine">Plans</p>
            <div className="mt-4 grid gap-4">
              {plans.map((plan) => (
                <div key={plan.id} className="rounded-3xl border border-ink/10 p-5">
                  <div className="flex items-baseline justify-between">
                    <h3 className="text-xl font-semibold">{plan.id}</h3>
                    <span className="text-sm font-semibold text-pine">{plan.price}</span>
                  </div>
                  <p className="mt-2 text-sm leading-6 text-ink/70">{plan.highlight}</p>
                </div>
              ))}
            </div>
          </div>
        </section>
      </div>
    </AppShell>
  );
}
