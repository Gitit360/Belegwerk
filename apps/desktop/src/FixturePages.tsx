import { useEffect, useState, type ReactNode } from "react";
import {
  ACCOUNTS_FIXTURE,
  ARCHIVE_FIXTURE,
  AUTOMATIONS_FIXTURE,
  MISSING_FIXTURE,
  REPORTS_FIXTURE,
  formatMoney,
} from "./fixtures/navPages";
import {
  EmptyState,
  FixtureFrame,
  type FixtureLoadState,
  fixtureStateCss,
} from "./uiStates";

/** Short synthetic load so empty/loading shells are visible in Vite/Tauri. */
function useFixtureReady(hasRows: boolean, fail = false): FixtureLoadState {
  const [state, setState] = useState<FixtureLoadState>("loading");
  useEffect(() => {
    const t = window.setTimeout(() => {
      if (fail) {
        setState("error");
      } else if (!hasRows) {
        setState("empty");
      } else {
        setState("ready");
      }
    }, 180);
    return () => window.clearTimeout(t);
  }, [hasRows, fail]);
  return state;
}

function FixtureShell({
  note,
  state,
  emptyTitle,
  emptyBody,
  children,
}: {
  note: string;
  state: FixtureLoadState;
  emptyTitle: string;
  emptyBody: string;
  children: ReactNode;
}) {
  return (
    <div>
      <p className="bw-fixture-note">{note}</p>
      <FixtureFrame
        state={state}
        emptyTitle={emptyTitle}
        emptyBody={emptyBody}
        loadingLabel="Fixture-Daten werden geladen …"
      >
        {children}
      </FixtureFrame>
      <style>{fixtureStateCss}</style>
    </div>
  );
}

export function AccountsPage() {
  const state = useFixtureReady(ACCOUNTS_FIXTURE.accounts.length > 0);
  return (
    <FixtureShell
      note={ACCOUNTS_FIXTURE.note}
      state={state}
      emptyTitle="Keine Konten"
      emptyBody="Es sind keine synthetischen Kontenfixtures geladen. Live-MoneyMoney bleibt macOS-only."
    >
      <ul className="bw-simple-list">
        {ACCOUNTS_FIXTURE.accounts.map((a) => (
          <li key={a.id}>
            <strong>{a.label}</strong>
            <div className="bw-list-meta">
              {a.provider} · {a.currency} · Read-only
            </div>
          </li>
        ))}
      </ul>
      <h3 className="bw-subhead">Letzte Bewegungen (Fixture)</h3>
      {ACCOUNTS_FIXTURE.recent.length === 0 ? (
        <EmptyState
          title="Keine Bewegungen"
          body="Für die geladenen Konten gibt es noch keine Fixture-Transaktionen."
        />
      ) : (
        <ul className="bw-simple-list">
          {ACCOUNTS_FIXTURE.recent.map((t) => (
            <li key={t.ref}>
              {t.bookingDate} · {formatMoney(t.amountMinor, "EUR")} · {t.counterparty}
              <div className="bw-list-meta">{t.purpose}</div>
            </li>
          ))}
        </ul>
      )}
    </FixtureShell>
  );
}

export function MissingPage() {
  const state = useFixtureReady(MISSING_FIXTURE.cases.length > 0);
  return (
    <FixtureShell
      note={MISSING_FIXTURE.note}
      state={state}
      emptyTitle="Keine offenen Fälle"
      emptyBody="Aktuell sind keine fehlenden Belege in den synthetischen Fixtures markiert."
    >
      <ul className="bw-simple-list">
        {MISSING_FIXTURE.cases.map((c) => (
          <li key={c.id}>
            <strong>{c.note}</strong>
            <div className="bw-list-meta">
              Eskalation {c.escalation} · {c.placeholder}
            </div>
          </li>
        ))}
      </ul>
    </FixtureShell>
  );
}

export function ArchivePage() {
  const state = useFixtureReady(ARCHIVE_FIXTURE.documents.length > 0);
  return (
    <FixtureShell
      note={ARCHIVE_FIXTURE.note}
      state={state}
      emptyTitle="Archiv leer"
      emptyBody="Noch keine content-addressierten Originale in der Fixture-Ansicht."
    >
      <ul className="bw-simple-list">
        {ARCHIVE_FIXTURE.documents.map((d) => (
          <li key={d.filename}>
            <strong>{d.filename}</strong>
            <div className="bw-list-meta">
              {d.hashPrefix} · {d.excerpt}
            </div>
          </li>
        ))}
      </ul>
    </FixtureShell>
  );
}

export function ReportsPage() {
  const state = useFixtureReady(REPORTS_FIXTURE.lines.length > 0);
  return (
    <FixtureShell
      note={REPORTS_FIXTURE.note}
      state={state}
      emptyTitle="Keine Berichtzeilen"
      emptyBody="Management-Basis ohne Daten — kein Steuerformular."
    >
      <ul className="bw-simple-list">
        {REPORTS_FIXTURE.lines.map((l) => (
          <li key={l.label}>
            <strong>{l.label}</strong> · {formatMoney(l.amountMinor, l.currency)}
            <div className="bw-list-meta">Drill-down: {l.drillDown}</div>
          </li>
        ))}
      </ul>
    </FixtureShell>
  );
}

export function AutomationsPage() {
  const state = useFixtureReady(AUTOMATIONS_FIXTURE.connectors.length > 0);
  return (
    <FixtureShell
      note="Connector-Stubs und blockierte M15-Erweiterungen — keine Live-Credentials."
      state={state}
      emptyTitle="Keine Connectoren"
      emptyBody="Es sind keine Connector-Stubs konfiguriert."
    >
      <h3 className="bw-subhead">Connectoren</h3>
      <ul className="bw-simple-list">
        {AUTOMATIONS_FIXTURE.connectors.map((c) => (
          <li key={c.id}>
            <strong>{c.label}</strong>
            <div className="bw-list-meta">
              {c.id} · {c.status}
            </div>
          </li>
        ))}
      </ul>
      <h3 className="bw-subhead">M15-Erweiterungen (blockiert/deferred)</h3>
      {AUTOMATIONS_FIXTURE.extensionsBlocked.length === 0 ? (
        <EmptyState title="Keine Blocker gelistet" body="Erweiterungs-Registry ist leer." />
      ) : (
        <p className="bw-list-meta">{AUTOMATIONS_FIXTURE.extensionsBlocked.join(" · ")}</p>
      )}
    </FixtureShell>
  );
}

export function AccountingPage() {
  const state = useFixtureReady(true);
  return (
    <FixtureShell
      note="SKR03-Platzhalter ohne Steuersätze — unverifizierte Fälle bleiben blockiert."
      state={state}
      emptyTitle="Keine Buchungsentwürfe"
      emptyBody="Es liegen keine journal drafts vor."
    >
      <ul className="bw-simple-list">
        <li>
          <strong>Bankzahlungen</strong>
          <div className="bw-list-meta">Dauerhaft gesperrt (Permission Gate).</div>
        </li>
        <li>
          <strong>Regelpaket</strong>
          <div className="bw-list-meta">
            Drafts ohne reviewed/active Pack → <code>BlockedUnsupported</code>
          </div>
        </li>
        <li>
          <strong>EU-B2B / ERiC</strong>
          <div className="bw-list-meta">Außerhalb v1 — siehe OPEN_QUESTIONS (externe Blocker).</div>
        </li>
      </ul>
    </FixtureShell>
  );
}

export function PlaceholderModulePage({ title }: { title: string }) {
  const state = useFixtureReady(false);
  return (
    <FixtureShell
      note={`Modulfläche „${title}“ — Fixture/Backend folgen.`}
      state={state}
      emptyTitle={`${title}: noch leer`}
      emptyBody="Siehe PROJECT_STATUS.md und docs/acceptance/MODULE_SURFACE.md."
    >
      {null}
    </FixtureShell>
  );
}
