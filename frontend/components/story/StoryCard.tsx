import { type Language, type StoryChapter } from "@/lib/types";

export function StoryCard({
  chapter,
  language
}: {
  chapter: StoryChapter;
  language: Language;
}) {
  return (
    <article className="rounded-[26px] border border-white/70 bg-white/90 p-5 shadow-panel transition hover:-translate-y-0.5">
      <div className="flex items-start justify-between gap-4">
        <div>
          <p className="text-xs uppercase tracking-[0.3em] text-copper">Chapter {chapter.chapter}</p>
          <h3 className="mt-1 font-[var(--font-display)] text-2xl font-semibold text-ink">
            {language === "te" ? chapter.titleTe : chapter.title}
          </h3>
        </div>
        <span className="rounded-full bg-pine/10 px-3 py-1 text-sm font-semibold text-pine">
          {chapter.score}
        </span>
      </div>
      <p className="mt-4 text-sm leading-7 text-ink/75">{chapter.text[language]}</p>
    </article>
  );
}
