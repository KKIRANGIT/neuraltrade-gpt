import { AppShell } from "@/components/layout/AppShell";
import { PageHeader } from "@/components/layout/PageHeader";
import {
  getLanguageFromSearchParam,
  getPlans,
  getUserProfile
} from "@/lib/api/stocksApi";

export default async function SettingsPage({
  searchParams
}: {
  searchParams?: Record<string, string | string[] | undefined>;
}) {
  const params = searchParams ?? {};
  const language = getLanguageFromSearchParam(
    Array.isArray(params.lang) ? params.lang[0] : params.lang
  );
  const [profile, plans] = await Promise.all([getUserProfile(), getPlans()]);

  return (
    <AppShell activePath="/settings" language={language}>
      <div className="space-y-6">
        <PageHeader
          eyebrow="Settings"
          title="Language, alerts, Telegram, and subscription controls"
          description="This page mirrors the product settings surface: language preference, quiet hours, alert channels, and subscription plan visibility."
        />
        <section className="grid gap-6 lg:grid-cols-[0.95fr_1.05fr]">
          <div className="space-y-6">
            <div className="rounded-[28px] border border-white/70 bg-white/90 p-6 shadow-panel">
              <p className="text-xs uppercase tracking-[0.3em] text-copper">Profile</p>
              <div className="mt-4 space-y-3 text-sm">
                <div className="flex justify-between">
                  <span>Name</span>
                  <span>{profile.name}</span>
                </div>
                <div className="flex justify-between">
                  <span>Email</span>
                  <span>{profile.email}</span>
                </div>
                <div className="flex justify-between">
                  <span>Plan</span>
                  <span>{profile.plan}</span>
                </div>
                <div className="flex justify-between">
                  <span>Language</span>
                  <span>{profile.language.toUpperCase()}</span>
                </div>
              </div>
            </div>
            <div className="rounded-[28px] border border-white/70 bg-white/90 p-6 shadow-panel">
              <p className="text-xs uppercase tracking-[0.3em] text-pine">Alerts</p>
              <ul className="mt-4 space-y-3 text-sm text-ink/75">
                {profile.alertTypes.map((alertType) => (
                  <li key={alertType}>{alertType}</li>
                ))}
              </ul>
              <p className="mt-4 text-sm text-ink/65">Quiet hours: {profile.quietHours}</p>
              <p className="mt-2 text-sm text-ink/65">
                Telegram: {profile.telegramConnected ? "Connected" : "Not connected"}
              </p>
            </div>
          </div>
          <div className="rounded-[28px] border border-white/70 bg-white/90 p-6 shadow-panel">
            <p className="text-xs uppercase tracking-[0.3em] text-copper">Subscription management</p>
            <div className="mt-4 grid gap-4">
              {plans.map((plan) => (
                <div
                  key={plan.id}
                  className={`rounded-3xl border p-5 ${
                    plan.id === profile.plan ? "border-pine bg-pine/5" : "border-ink/10"
                  }`}
                >
                  <div className="flex items-baseline justify-between">
                    <h3 className="text-xl font-semibold text-ink">{plan.id}</h3>
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
