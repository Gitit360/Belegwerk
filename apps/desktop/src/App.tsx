import { useState } from "react";
import { NAV_ITEMS, PLACEHOLDER_COPY, type NavId } from "./nav";

export function App() {
  const [active, setActive] = useState<NavId>("overview");
  const copy = PLACEHOLDER_COPY[active];

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
      <main className="bw-main bw-fade-in" key={active}>
        <header className="bw-main-header">
          <h2>{copy.title}</h2>
          <p className="bw-lede">{copy.body}</p>
        </header>
        <section className="bw-panel" aria-label="Platzhalter">
          <p>
            Scaffold-Status: Domäne, Permission Gate, SQLite-Migrationen und Plugin-SDK sind
            vorbereitet. Fachliche Steuerregeln werden nicht erfunden — unsupported cases bleiben
            blockiert.
          </p>
          <p className="bw-meta">
            Theme: Tokens hell/dunkel bereit · Keine Telemetrie · Synthetische Fixtures only
          </p>
        </section>
      </main>
      <style>{shellCss}</style>
    </div>
  );
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
