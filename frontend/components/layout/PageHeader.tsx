export function PageHeader({
  eyebrow,
  title,
  description,
  action
}: {
  eyebrow: string;
  title: string;
  description: string;
  action?: React.ReactNode;
}) {
  return (
    <div className="flex flex-col gap-4 rounded-[30px] border border-white/70 bg-white/85 p-6 shadow-panel sm:p-8 lg:flex-row lg:items-end lg:justify-between">
      <div className="max-w-2xl space-y-2">
        <p className="text-xs font-semibold uppercase tracking-[0.3em] text-copper">{eyebrow}</p>
        <h2 className="font-[var(--font-display)] text-3xl font-semibold text-ink sm:text-4xl">
          {title}
        </h2>
        <p className="text-sm leading-7 text-ink/70 sm:text-base">{description}</p>
      </div>
      {action ? <div className="shrink-0">{action}</div> : null}
    </div>
  );
}
