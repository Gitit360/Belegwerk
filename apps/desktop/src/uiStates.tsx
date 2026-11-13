import type { ReactNode } from "react";

/** Shared German empty / loading / error shells using design tokens. */

export type FixtureLoadState = "loading" | "ready" | "empty" | "error";

export function LoadingState({ label = "Daten werden geladen …" }: { label?: string }) {
  return (
    <div className="bw-state bw-state-loading" role="status" aria-live="polite">
      <span className="bw-state-pulse" aria-hidden />
      <p>{label}</p>
    </div>
  );
}

export function EmptyState({
  title = "Keine Einträge",
  body = "Für diese Ansicht liegen noch keine synthetischen Fixtures vor.",
}: {
  title?: string;
  body?: string;
}) {
  return (
    <div className="bw-state bw-state-empty" role="status">
      <p className="bw-state-title">{title}</p>
      <p className="bw-state-body">{body}</p>
    </div>
  );
}

export function ErrorState({
  title = "Ansicht nicht verfügbar",
  body = "Die Fixture-Daten konnten nicht gelesen werden. Keine Live-Systeme betroffen.",
}: {
  title?: string;
  body?: string;
}) {
  return (
    <div className="bw-state bw-state-error" role="alert">
      <p className="bw-state-title">{title}</p>
      <p className="bw-state-body">{body}</p>
    </div>
  );
}

export function FixtureFrame({
  state,
  children,
  emptyTitle,
  emptyBody,
  errorBody,
  loadingLabel,
}: {
  state: FixtureLoadState;
  children: ReactNode;
  emptyTitle?: string;
  emptyBody?: string;
  errorBody?: string;
  loadingLabel?: string;
}) {
  if (state === "loading") {
    return <LoadingState label={loadingLabel} />;
  }
  if (state === "error") {
    return <ErrorState body={errorBody} />;
  }
  if (state === "empty") {
    return <EmptyState title={emptyTitle} body={emptyBody} />;
  }
  return <>{children}</>;
}

export const fixtureStateCss = `
.bw-state {
  margin-top: var(--bw-space-3);
  padding: var(--bw-space-6);
  border-radius: var(--bw-radius-md);
  border: 1px dashed var(--bw-color-border-subtle);
  background: var(--color-surface-bright);
  display: grid;
  gap: var(--bw-space-2);
  justify-items: start;
}
.bw-state-title {
  margin: 0;
  font-weight: 600;
  color: var(--bw-color-text-primary);
}
.bw-state-body, .bw-state p {
  margin: 0;
  color: var(--bw-color-text-secondary);
  font-size: 0.95rem;
  line-height: 1.5;
}
.bw-state-loading {
  border-style: solid;
  border-color: color-mix(in srgb, var(--color-accent) 45%, var(--bw-color-border-subtle));
}
.bw-state-empty {
  border-color: color-mix(in srgb, var(--bw-color-text-secondary) 25%, var(--bw-color-border-subtle));
}
.bw-state-error {
  border-style: solid;
  border-color: var(--color-message-error-border);
  background: color-mix(in srgb, var(--color-message-error-bg) 12%, var(--color-surface-raised));
}
.bw-state-error .bw-state-title {
  color: var(--color-message-error-bg);
}
.bw-state-pulse {
  width: 0.65rem;
  height: 0.65rem;
  border-radius: var(--bw-radius-pill);
  background: var(--color-accent);
}
@media (prefers-reduced-motion: no-preference) {
  .bw-state-loading {
    animation: busy-pulse-soft 1.6s ease-in-out infinite;
  }
  .bw-state-pulse {
    animation: busy-pulse 1.5s ease-in-out infinite;
  }
}
`;
