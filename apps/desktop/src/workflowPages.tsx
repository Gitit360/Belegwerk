import { useCallback, useEffect, useState, type DragEvent } from "react";
import { api, type TaxProfile } from "./api/client";
import { EmptyState, ErrorState, LoadingState, fixtureStateCss } from "./uiStates";

const pageCss = `
.bw-drop {
  margin-top: var(--bw-space-3);
  padding: var(--bw-space-8) var(--bw-space-4);
  border: 2px dashed var(--bw-color-border-subtle);
  border-radius: var(--bw-radius-md);
  background: var(--bw-color-surface-muted);
  text-align: center;
  color: var(--bw-color-text-secondary);
  transition: border-color var(--bw-motion-duration-short) var(--bw-motion-ease),
    background-color var(--bw-motion-duration-short) var(--bw-motion-ease);
}
.bw-drop.is-over {
  border-color: var(--bw-color-accent);
  background: var(--bw-color-surface-selected);
  color: var(--bw-color-text-primary);
}
.bw-btn {
  border: 1px solid var(--bw-color-border-subtle);
  background: var(--bw-color-surface-elevated);
  color: var(--bw-color-text-primary);
  border-radius: var(--bw-radius-sm);
  padding: var(--bw-space-2) var(--bw-space-3);
  cursor: pointer;
}
.bw-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.bw-btn-accent {
  background: var(--bw-color-accent);
  border-color: var(--bw-color-accent);
  color: var(--bw-color-text-inverse);
}
.bw-actions { display: flex; flex-wrap: wrap; gap: var(--bw-space-2); margin-top: var(--bw-space-3); }
.bw-form { display: grid; gap: var(--bw-space-3); margin-top: var(--bw-space-3); max-width: 28rem; }
.bw-form label { display: grid; gap: var(--bw-space-1); font-size: 0.9rem; }
.bw-form select, .bw-form input[type="number"] {
  font: inherit;
  padding: var(--bw-space-2);
  border-radius: var(--bw-radius-sm);
  border: 1px solid var(--bw-color-border-subtle);
  background: var(--bw-color-surface-elevated);
  color: var(--bw-color-text-primary);
}
.bw-viewer {
  margin-top: var(--bw-space-4);
  padding: var(--bw-space-4);
  border: 1px solid var(--bw-color-border-subtle);
  border-radius: var(--bw-radius-md);
  background: var(--bw-color-surface-muted);
}
.bw-viewer pre {
  white-space: pre-wrap;
  font-family: var(--bw-font-mono);
  font-size: 0.85rem;
  margin: var(--bw-space-2) 0 0;
}
.bw-warn { color: var(--bw-color-status-warning); }
.bw-ok { color: var(--bw-color-status-success); }
.bw-danger { color: var(--bw-color-status-danger); }
`;

type Batch = {
  id: string;
  sourceLabel?: string;
  source_label?: string;
  importedAt?: string;
  imported_at?: string;
  documentCount?: number;
  document_count?: number;
};

type Doc = {
  id: string;
  contentHash?: string;
  content_hash?: string;
  originalFilename?: string | null;
  original_filename?: string | null;
  mimeType?: string;
  mime_type?: string;
  byteSize?: number;
  byte_size?: number;
  text?: string;
};

export function InboxPage() {
  const [batches, setBatches] = useState<Batch[]>([]);
  const [docs, setDocs] = useState<Doc[]>([]);
  const [selectedBatch, setSelectedBatch] = useState<string | null>(null);
  const [viewer, setViewer] = useState<Doc | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [over, setOver] = useState(false);
  const [busy, setBusy] = useState(false);

  const reload = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const b = (await api.importListBatches()) as Batch[];
      setBatches(b);
      const d = (await api.importListDocuments(selectedBatch ?? undefined)) as Doc[];
      setDocs(d);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Import konnte nicht geladen werden");
    } finally {
      setLoading(false);
    }
  }, [selectedBatch]);

  useEffect(() => {
    void reload();
  }, [reload]);

  async function ingestFiles(fileList: FileList | File[]) {
    const files = Array.from(fileList);
    if (files.length === 0) return;
    setBusy(true);
    setError(null);
    try {
      const payload = await Promise.all(
        files.map(
          (f) =>
            new Promise<{ filename: string; mimeType: string; textContent: string }>(
              (resolve, reject) => {
                const reader = new FileReader();
                reader.onload = () =>
                  resolve({
                    filename: f.name,
                    mimeType: f.type || "text/plain",
                    textContent: String(reader.result ?? ""),
                  });
                reader.onerror = () => reject(reader.error ?? new Error("read failed"));
                reader.readAsText(f);
              },
            ),
        ),
      );
      await api.importFiles("dropzone", payload);
      setSelectedBatch(null);
      await reload();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Import fehlgeschlagen");
    } finally {
      setBusy(false);
    }
  }

  function onDrop(e: DragEvent) {
    e.preventDefault();
    setOver(false);
    void ingestFiles(e.dataTransfer.files);
  }

  if (loading && batches.length === 0) {
    return (
      <div>
        <LoadingState label="Eingang wird geladen …" />
        <style>{fixtureStateCss}</style>
      </div>
    );
  }

  return (
    <div>
      <p className="bw-fixture-note">
        Drop-Zone importiert synthetische Dateien in die ImportPipeline (Browser-Fallback oder
        Tauri).
      </p>
      {error ? <ErrorState title="Import-Fehler" body={error} /> : null}
      <div
        className={`bw-drop${over ? " is-over" : ""}`}
        onDragOver={(e) => {
          e.preventDefault();
          setOver(true);
        }}
        onDragLeave={() => setOver(false)}
        onDrop={onDrop}
      >
        <p>Dateien hierher ziehen oder auswählen (Text/JSON/Fixtures).</p>
        <label className="bw-btn bw-btn-accent" style={{ display: "inline-block", marginTop: "0.75rem" }}>
          Dateien wählen
          <input
            type="file"
            multiple
            hidden
            disabled={busy}
            onChange={(e) => {
              if (e.target.files) void ingestFiles(e.target.files);
            }}
          />
        </label>
      </div>

      <h3 className="bw-subhead">ImportBatches</h3>
      {batches.length === 0 ? (
        <EmptyState title="Keine Batches" body="Noch keine Importe — Drop-Zone nutzen." />
      ) : (
        <ul className="bw-simple-list">
          {batches.map((b) => (
            <li key={b.id}>
              <button
                type="button"
                className="bw-btn"
                onClick={() => setSelectedBatch(b.id)}
              >
                <strong>{b.sourceLabel ?? b.source_label}</strong>
              </button>
              <div className="bw-list-meta">
                {b.importedAt ?? b.imported_at} · {b.documentCount ?? b.document_count ?? 0}{" "}
                Dokumente
              </div>
            </li>
          ))}
        </ul>
      )}

      <h3 className="bw-subhead">Dokumente {selectedBatch ? "(Batch)" : "(zuletzt)"}</h3>
      {docs.length === 0 ? (
        <EmptyState title="Keine Dokumente" body="Importierte Originale erscheinen hier." />
      ) : (
        <ul className="bw-simple-list">
          {docs.map((d) => (
            <li key={d.id}>
              <button type="button" className="bw-btn" onClick={() => setViewer(d)}>
                {d.originalFilename ?? d.original_filename ?? d.id}
              </button>
              <div className="bw-list-meta">
                {(d.contentHash ?? d.content_hash ?? "").slice(0, 18)}… ·{" "}
                {d.byteSize ?? d.byte_size} B · {d.mimeType ?? d.mime_type}
              </div>
            </li>
          ))}
        </ul>
      )}

      {viewer ? (
        <div className="bw-viewer" aria-label="Dokumenten-Viewer Platzhalter">
          <strong>{viewer.originalFilename ?? viewer.original_filename ?? viewer.id}</strong>
          <div className="bw-list-meta">Viewer-Platzhalter — Original bytes im Object Store</div>
          <pre>{viewer.text ?? "(Inhalt nur im Browser-Fallback / nach Text-Import sichtbar)"}</pre>
        </div>
      ) : null}
      <style>{pageCss}</style>
      <style>{fixtureStateCss}</style>
    </div>
  );
}

type CaseRow = {
  id: string;
  note: string;
  escalationLevel?: number;
  escalation_level?: number;
  status: string;
  createdAt?: string;
  created_at?: string;
};

export function MissingWorkflowPage() {
  const [rows, setRows] = useState<CaseRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function reload() {
    setLoading(true);
    setError(null);
    try {
      setRows((await api.remindersList()) as CaseRow[]);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Fälle konnten nicht geladen werden");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void reload();
  }, []);

  async function escalate(id: string) {
    setBusy(true);
    try {
      await api.remindersEscalate(id);
      await reload();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Eskalation fehlgeschlagen");
    } finally {
      setBusy(false);
    }
  }

  async function complete(id: string) {
    setBusy(true);
    try {
      await api.remindersComplete(id);
      await reload();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Abschließen fehlgeschlagen");
    } finally {
      setBusy(false);
    }
  }

  if (loading) {
    return (
      <div>
        <LoadingState label="Fehlende Belege werden geladen …" />
        <style>{fixtureStateCss}</style>
      </div>
    );
  }

  return (
    <div>
      <p className="bw-fixture-note">
        ReminderService — Eskalation und Abschluss sind Permission-Gate-geschützt.
      </p>
      {error ? <ErrorState title="Reminder-Fehler" body={error} /> : null}
      {rows.length === 0 ? (
        <EmptyState title="Keine offenen Fälle" body="Alle MissingDocumentCases sind erledigt." />
      ) : (
        <ul className="bw-simple-list">
          {rows.map((c) => (
            <li key={c.id}>
              <strong>{c.note || "Ohne Notiz"}</strong>
              <div className="bw-list-meta">
                {c.status} · Eskalation {c.escalationLevel ?? c.escalation_level ?? 0} ·{" "}
                {c.createdAt ?? c.created_at}
              </div>
              <div className="bw-actions">
                <button
                  type="button"
                  className="bw-btn"
                  disabled={busy}
                  onClick={() => void escalate(c.id)}
                >
                  Eskalieren
                </button>
                <button
                  type="button"
                  className="bw-btn bw-btn-accent"
                  disabled={busy}
                  onClick={() => void complete(c.id)}
                >
                  Abschließen
                </button>
              </div>
            </li>
          ))}
        </ul>
      )}
      <style>{pageCss}</style>
      <style>{fixtureStateCss}</style>
    </div>
  );
}

type JournalRow = {
  id: string;
  status: string;
  description?: string | null;
  blockedUnsupported?: boolean;
  blocked_unsupported?: boolean;
  blockReason?: string | null;
  block_reason?: string | null;
  rulePackStatus?: string | null;
  rule_pack_status?: string | null;
  lines: Array<{
    account_code?: string;
    accountCode?: string;
    debit_minor?: number;
    debitMinor?: number;
    credit_minor?: number;
    creditMinor?: number;
    currency: string;
  }>;
};

export function AccountingWorkflowPage() {
  const [rows, setRows] = useState<JournalRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [msg, setMsg] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function reload() {
    setLoading(true);
    setError(null);
    try {
      setRows((await api.journalList()) as JournalRow[]);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Entwürfe konnten nicht geladen werden");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void reload();
  }, []);

  async function approve(id: string) {
    setBusy(true);
    setMsg(null);
    setError(null);
    try {
      await api.journalGrantApproval(id);
      const m = await api.journalApprove(id);
      setMsg(String(m));
      await reload();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Freigabe fehlgeschlagen");
    } finally {
      setBusy(false);
    }
  }

  if (loading) {
    return (
      <div>
        <LoadingState label="Buchungsentwürfe werden geladen …" />
        <style>{fixtureStateCss}</style>
      </div>
    );
  }

  return (
    <div>
      <p className="bw-fixture-note">
        Journal-Entwürfe — Freigabe nur über Permission Gate (BookingPost + Approval). Keine
        erfundenen Steuersätze.
      </p>
      {error ? <ErrorState title="Buchhaltung" body={error} /> : null}
      {msg ? <p className="bw-ok">{msg}</p> : null}
      {rows.length === 0 ? (
        <EmptyState title="Keine Entwürfe" body="Noch keine journal drafts." />
      ) : (
        <ul className="bw-simple-list">
          {rows.map((r) => {
            const blocked = r.blockedUnsupported ?? r.blocked_unsupported;
            return (
              <li key={r.id}>
                <strong>{r.id}</strong> · {r.status}
                <div className="bw-list-meta">{r.description}</div>
                {blocked ? (
                  <p className="bw-danger">
                    Blockiert: {r.blockReason ?? r.block_reason ?? "unsupported"}
                  </p>
                ) : (
                  <p className="bw-list-meta">
                    Regelpaket: {r.rulePackStatus ?? r.rule_pack_status ?? "—"}
                  </p>
                )}
                <ul className="bw-list-meta">
                  {r.lines.map((l, i) => (
                    <li key={i}>
                      {l.accountCode ?? l.account_code}:{" "}
                      {l.debitMinor ?? l.debit_minor ?? 0}/{l.creditMinor ?? l.credit_minor ?? 0}{" "}
                      {l.currency}
                    </li>
                  ))}
                </ul>
                <div className="bw-actions">
                  <button
                    type="button"
                    className="bw-btn bw-btn-accent"
                    disabled={busy || Boolean(blocked) || r.status === "approved"}
                    onClick={() => void approve(r.id)}
                  >
                    Freigabe (Gate)
                  </button>
                </div>
              </li>
            );
          })}
        </ul>
      )}
      <style>{pageCss}</style>
      <style>{fixtureStateCss}</style>
    </div>
  );
}

export function SettingsWorkflowPage() {
  const [profile, setProfile] = useState<TaxProfile | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    void (async () => {
      try {
        setProfile(await api.settingsGet());
      } catch (e) {
        setError(e instanceof Error ? e.message : "Einstellungen nicht ladbar");
      } finally {
        setLoading(false);
      }
    })();
  }, []);

  async function save() {
    if (!profile) return;
    setBusy(true);
    setSaved(false);
    setError(null);
    try {
      const next = await api.settingsSave(profile);
      setProfile(next);
      setSaved(true);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Speichern fehlgeschlagen");
    } finally {
      setBusy(false);
    }
  }

  if (loading) {
    return (
      <div>
        <LoadingState label="Einstellungen werden geladen …" />
        <style>{fixtureStateCss}</style>
      </div>
    );
  }
  if (!profile) {
    return (
      <div>
        <ErrorState title="Einstellungen" body={error ?? "Kein TaxProfile"} />
        <style>{fixtureStateCss}</style>
      </div>
    );
  }

  return (
    <div>
      <p className="bw-fixture-note">
        TaxProfile lokal in SQLite — Defaults: monatliche Istversteuerung, Dauerfrist aus. Kein
        ELSTER.
      </p>
      {error ? <ErrorState title="Speichern" body={error} /> : null}
      {saved ? <p className="bw-ok">Gespeichert.</p> : null}
      <div className="bw-form">
        <label>
          USt-Verfahren
          <select
            value={profile.vat_method}
            onChange={(e) =>
              setProfile({ ...profile, vat_method: e.target.value as TaxProfile["vat_method"] })
            }
          >
            <option value="ist">Istversteuerung</option>
            <option value="soll">Sollversteuerung</option>
          </select>
        </label>
        <label>
          Melde-Rhythmus
          <select
            value={profile.reporting_cadence}
            onChange={(e) =>
              setProfile({
                ...profile,
                reporting_cadence: e.target.value as TaxProfile["reporting_cadence"],
              })
            }
          >
            <option value="monthly">Monatlich</option>
            <option value="quarterly">Quartalsweise</option>
          </select>
        </label>
        <label>
          Steuerjahr
          <input
            type="number"
            value={profile.tax_year}
            onChange={(e) => setProfile({ ...profile, tax_year: Number(e.target.value) })}
          />
        </label>
        <label>
          <input
            type="checkbox"
            checked={profile.dauerfrist_extension}
            onChange={(e) =>
              setProfile({ ...profile, dauerfrist_extension: e.target.checked })
            }
          />{" "}
          Dauerfristverlängerung (Standard aus)
        </label>
        <button type="button" className="bw-btn bw-btn-accent" disabled={busy} onClick={() => void save()}>
          Speichern
        </button>
      </div>
      <style>{pageCss}</style>
      <style>{fixtureStateCss}</style>
    </div>
  );
}
