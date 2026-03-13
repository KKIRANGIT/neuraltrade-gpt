import Link from "next/link";
import { type Language } from "@/lib/types";

const navItems = [
  { href: "/", label: "Home" },
  { href: "/dashboard", label: "Dashboard" },
  { href: "/stocks", label: "Stocks" },
  { href: "/signals", label: "Signals" },
  { href: "/settings", label: "Settings" }
];

export function AppShell({
  activePath,
  languagePath,
  language,
  children
}: {
  activePath: string;
  languagePath?: string;
  language: Language;
  children: React.ReactNode;
}) {
  const togglePath = languagePath ?? activePath;

  return (
    <div className="min-h-screen">
      <div className="mx-auto flex min-h-screen w-full max-w-7xl flex-col px-4 py-5 sm:px-6 lg:px-8">
        <header className="sticky top-4 z-20 rounded-[28px] border border-white/70 bg-white/80 px-5 py-4 shadow-panel backdrop-blur">
          <div className="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
            <div>
              <p className="text-xs font-semibold uppercase tracking-[0.35em] text-pine">
                NeuralTrade
              </p>
              <h1 className="font-[var(--font-display)] text-2xl font-semibold text-ink">
                AI market intelligence for Indian equities
              </h1>
            </div>
            <div className="flex flex-wrap items-center gap-3">
              <nav className="flex flex-wrap gap-2 text-sm font-medium">
                {navItems.map((item) => {
                  const active = activePath === item.href;
                  return (
                    <Link
                      key={item.href}
                      href={`${item.href}${item.href.includes("?") ? "&" : "?"}lang=${language}`}
                      className={`rounded-full px-4 py-2 transition ${
                        active
                          ? "bg-ink text-white"
                          : "bg-ink/5 text-ink hover:bg-ink/10"
                      }`}
                    >
                      {item.label}
                    </Link>
                  );
                })}
              </nav>
              <div className="rounded-full border border-ink/10 bg-mist px-2 py-1 text-sm">
                <Link
                  href={`${togglePath}?lang=en`}
                  className={`rounded-full px-3 py-1 ${language === "en" ? "bg-white shadow-sm" : ""}`}
                >
                  EN
                </Link>
                <Link
                  href={`${togglePath}?lang=te`}
                  className={`rounded-full px-3 py-1 ${language === "te" ? "bg-white shadow-sm" : ""}`}
                >
                  TE
                </Link>
              </div>
            </div>
          </div>
        </header>
        <main className="flex-1 py-6">{children}</main>
      </div>
    </div>
  );
}
