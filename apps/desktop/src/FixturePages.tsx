import {
  ACCOUNTS_FIXTURE,
  ARCHIVE_FIXTURE,
  AUTOMATIONS_FIXTURE,
  MISSING_FIXTURE,
  REPORTS_FIXTURE,
  formatMoney,
} from "./fixtures/navPages";

export function AccountsPage() {
  return (
    <div>
      <p className="bw-fixture-note">{ACCOUNTS_FIXTURE.note}</p>
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
      <ul className="bw-simple-list">
        {ACCOUNTS_FIXTURE.recent.map((t) => (
          <li key={t.ref}>
            {t.bookingDate} · {formatMoney(t.amountMinor, "EUR")} · {t.counterparty}
            <div className="bw-list-meta">{t.purpose}</div>
          </li>
        ))}
      </ul>
    </div>
  );
}

export function MissingPage() {
  return (
    <div>
      <p className="bw-fixture-note">{MISSING_FIXTURE.note}</p>
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
    </div>
  );
}

export function ArchivePage() {
  return (
    <div>
      <p className="bw-fixture-note">{ARCHIVE_FIXTURE.note}</p>
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
    </div>
  );
}

export function ReportsPage() {
  return (
    <div>
      <p className="bw-fixture-note">{REPORTS_FIXTURE.note}</p>
      <ul className="bw-simple-list">
        {REPORTS_FIXTURE.lines.map((l) => (
          <li key={l.label}>
            <strong>{l.label}</strong> · {formatMoney(l.amountMinor, l.currency)}
            <div className="bw-list-meta">Drill-down: {l.drillDown}</div>
          </li>
        ))}
      </ul>
    </div>
  );
}

export function AutomationsPage() {
  return (
    <div>
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
      <p className="bw-list-meta">{AUTOMATIONS_FIXTURE.extensionsBlocked.join(" · ")}</p>
    </div>
  );
}
