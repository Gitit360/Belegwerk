import { useEffect, useState } from "react";
import {
  AccountsPage,
  ArchivePage,
  AutomationsPage,
  MissingPage,
  ReportsPage,
} from "./FixturePages";
import { MatchingWorkbench } from "./MatchingWorkbench";
import { NAV_ITEMS, PLACEHOLDER_COPY, type NavId } from "./nav";

type HealthInfo = {
  app: string;
  version: string;
  locale: string;
  bank_payments: boolean;
  telemetry_default: boolean;
};

const FALLBACK_HEALTH: HealthInfo = {
  app: "Belegwerk",
  version: "0.1.0",
  locale: "de-DE",
  bank_payments: false,
  telemetry_default: false,
};

async function loadHealth(): Promise<HealthInfo> {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke<HealthInfo>("health");
  } catch {
    return FALLBACK_HEALTH;
  }
}

export function App() {
  const [active, setActive] = useState<NavId>("overview");
  const [health, setHealth] = useState<HealthInfo>(FALLBACK_HEALTH);
  const copy = PLACEHOLDER_COPY[active];

  useEffect(() => {
    let cancelled = false;
    void loadHealth().then((h) => {
      if (!cancelled) setHealth(h);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <div className="bw-shell">
      <aside className="bw-nav" aria-label="Hauptnavigation">
        <div className="bw-brand">
          <p className="bw-brand-eyebrow">Lokal · EÜR</p>
          <h1 className="bw-brand-name">Belegwerk</h1>
        </div>
        <nav>
          <ul>
            {NAV_ITEMS.map((item) => (
              <li key={item.id}>
                <button
                  type="button"
                  className={`bw-nav-item${active === item.id ? " is-active" : ""}`}
                  aria-current={active === item.id ? "page" : undefined}
                  onClick={() => setActive(item.id)}
                >
                  {item.label}
                </button>
              </li>
            ))}
          </ul>
        </nav>
      </aside>
      <main
        className={`bw-main bw-fade-in${active === "matching" ? " is-wide" : ""}`}
        key={active}
      >
        <header className="bw-main-header">
          <h2>{copy.title}</h2>
          <p className="bw-lede">{copy.body}</p>
        </header>
        <section className="bw-panel" aria-label="Modulstatus">
          <ModulePanel id={active} />
          <p className="bw-meta">
            {health.app} {health.version} · {health.locale} · Bankzahlungen:{" "}
            {health.bank_payments ? "an" : "aus"} · Telemetrie-Default:{" "}
            {health.telemetry_default ? "an" : "aus"}
          </p>
        </section>
      </main>
      <style>{shellCss}</style>
    </div>
  );
}

function ModulePanel({ id }: { id: NavId }) {
  switch (id) {
    case "overview":
      return (
        <p>
          Scaffold-Status: Domäne, Permission Gate, SQLite-Migrationen und Plugin-SDK sind
          vorbereitet. Fachliche Steuerregeln werden nicht erfunden — unsupported cases bleiben
          blockiert.
        </p>
      );
    case "accounts":
      return <AccountsPage />;
    case "matching":
      return <MatchingWorkbench />;
    case "missing":
      return <MissingPage />;
    case "archive":
      return <ArchivePage />;
    case "accounting":
      return (
        <p>
          SKR03-Kontenrahmen-Platzhalter ohne Steuersätze. Buchungsentwürfe ohne freigegebenes
          Regelpaket sind nicht festschreibbar (<code>BlockedUnsupported</code>). Bankzahlungen
          bleiben dauerhaft gesperrt.
        </p>
      );
    case "reports":
      return <ReportsPage />;
    case "automations":
      return <AutomationsPage />;
    default:
      return (
        <p>
          Platzhalter für Modulfläche <strong>{PLACEHOLDER_COPY[id].title}</strong>. Siehe{" "}
          <code>PROJECT_STATUS.md</code> und <code>docs/acceptance/MODULE_SURFACE.md</code>.
        </p>
      );
  }
}

const shellCss = `
.bw-shell {
  display: grid;
  grid-template-columns: var(--bw-size-nav-width) 1fr;
  min-height: 100%;
}
.bw-nav {
  padding: var(--bw-space-6) var(--bw-space-4);
  border-right: 1px solid var(--bw-color-border-subtle);
  background: color-mix(in srgb, var(--bw-color-surface-elevated) 88%, transparent);
  backdrop-filter: blur(10px);
}
.bw-brand-eyebrow {
  margin: 0;
  font-size: 0.75rem;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--bw-color-text-secondary);
}
.bw-brand-name {
  margin: var(--bw-space-1) 0 var(--bw-space-6);
  font-family: var(--bw-font-display);
  font-weight: 700;
  font-size: 1.85rem;
  letter-spacing: -0.02em;
}
.bw-nav ul {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: var(--bw-space-1);
}
.bw-nav-item {
  width: 100%;
  text-align: left;
  border: 0;
  background: transparent;
  color: var(--bw-color-text-secondary);
  padding: var(--bw-space-2) var(--bw-space-3);
  border-radius: var(--bw-radius-sm);
  cursor: pointer;
}
.bw-nav-item:hover {
  background: var(--bw-color-surface-muted);
  color: var(--bw-color-text-primary);
}
.bw-nav-item.is-active {
  background: var(--bw-color-surface-selected);
  color: var(--bw-color-text-primary);
  font-weight: 600;
}
.bw-main {
  padding: var(--bw-space-8);
  max-width: 52rem;
}
.bw-main.is-wide {
  max-width: 72rem;
}
.bw-main-header h2 {
  margin: 0 0 var(--bw-space-2);
  font-family: var(--bw-font-display);
  font-size: 2rem;
}
.bw-lede {
  margin: 0;
  color: var(--bw-color-text-secondary);
  line-height: 1.5;
}
.bw-panel {
  margin-top: var(--bw-space-6);
  padding: var(--bw-space-6);
  background: var(--bw-color-surface-elevated);
  border: 1px solid var(--bw-color-border-subtle);
  border-radius: var(--bw-radius-md);
  line-height: 1.55;
}
.bw-meta {
  margin-top: var(--bw-space-4);
  color: var(--bw-color-text-secondary);
  font-size: 0.9rem;
}
.bw-fixture-note, .bw-list-meta {
  color: var(--bw-color-text-secondary);
  font-size: 0.9rem;
}
.bw-simple-list {
  list-style: none;
  margin: var(--bw-space-3) 0 0;
  padding: 0;
  display: grid;
  gap: var(--bw-space-2);
}
.bw-simple-list li {
  padding: var(--bw-space-3);
  border: 1px solid var(--bw-color-border-subtle);
  border-radius: var(--bw-radius-sm);
  background: var(--bw-color-surface-muted);
}
.bw-subhead {
  margin: var(--bw-space-4) 0 var(--bw-space-2);
  font-size: 0.95rem;
}
code {
  font-family: var(--bw-font-mono);
  font-size: 0.9em;
}
@media (max-width: 800px) {
  .bw-shell {
    grid-template-columns: 1fr;
  }
  .bw-nav {
    border-right: 0;
    border-bottom: 1px solid var(--bw-color-border-subtle);
  }
}
`;
