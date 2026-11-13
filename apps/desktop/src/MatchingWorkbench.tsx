import { useEffect, useMemo, useState } from "react";
import { api } from "./api/client";
import { formatMoney } from "./fixtures/matchingWorkbench";
import { EmptyState, ErrorState, LoadingState, fixtureStateCss } from "./uiStates";

type Snap = {
  note: string;
  transactions: Array<{
    id: string;
    bookingDate: string;
    amountMinor: number;
    currency: string;
    description: string;
    counterparty: string;
    status: string;
    allocatedMinor: number;
    openRemainderMinor: number;
  }>;
  documents: Array<{
    id: string;
    filename: string;
    invoiceRef: string;
    amountMinor: number;
    currency: string;
    excerpt: string;
  }>;
  suggestions: Array<{
    transactionId: string;
    documentId: string;
    confidence: string;
    reason: string;
    decision?: string | null;
  }>;
};

type Preview = {
  selected_count?: number;
  selectedCount?: number;
  confirmable_count?: number;
  confirmableCount?: number;
  exception_count?: number;
  exceptionCount?: number;
  sum_allocated_minor?: number;
  sumAllocatedMinor?: number;
  sum_open_remainder_minor?: number;
  sumOpenRemainderMinor?: number;
  exceptions: string[];
};

export function MatchingWorkbench() {
  const [snap, setSnap] = useState<Snap | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [selectedTxId, setSelectedTxId] = useState("");
  const [selectedPairs, setSelectedPairs] = useState<Record<string, boolean>>({});
  const [preview, setPreview] = useState<Preview | null>(null);
  const [busy, setBusy] = useState(false);
  const [privatePercent, setPrivatePercent] = useState(0);

  async function reload() {
    setLoading(true);
    setError(null);
    try {
      const data = (await api.matchingSnapshot()) as Snap;
      setSnap(data);
      setSelectedTxId((prev) => prev || data.transactions[0]?.id || "");
    } catch (e) {
      setError(e instanceof Error ? e.message : "Abgleich konnte nicht geladen werden");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void reload();
  }, []);

  const selectedTx = useMemo(
    () => snap?.transactions.find((t) => t.id === selectedTxId),
    [snap, selectedTxId],
  );
  const suggestion = snap?.suggestions.find((s) => s.transactionId === selectedTxId);
  const linkedDoc = suggestion
    ? snap?.documents.find((d) => d.id === suggestion.documentId)
    : undefined;

  async function confirm() {
    if (!suggestion) return;
    setBusy(true);
    try {
      await api.matchingConfirm(
        suggestion.transactionId,
        suggestion.documentId,
        privatePercent,
      );
      await reload();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Bestätigen fehlgeschlagen");
    } finally {
      setBusy(false);
    }
  }

  async function rebuild() {
    setBusy(true);
    try {
      await api.matchingRebuild();
      await reload();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Vorschläge neu berechnen fehlgeschlagen");
    } finally {
      setBusy(false);
    }
  }

  async function reject() {
    if (!suggestion) return;
    setBusy(true);
    try {
      await api.matchingReject(suggestion.transactionId, suggestion.documentId);
      await reload();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Ablehnen fehlgeschlagen");
    } finally {
      setBusy(false);
    }
  }

  async function runPreview() {
    const items = Object.entries(selectedPairs)
      .filter(([, on]) => on)
      .map(([k]) => {
        const [transactionId, documentId] = k.split("::");
        return { transactionId, documentId };
      });
    setBusy(true);
    try {
      const p = (await api.matchingBatchPreview(items)) as Preview;
      setPreview(p);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Vorschau fehlgeschlagen");
    } finally {
      setBusy(false);
    }
  }

  async function confirmSelected() {
    const items = Object.entries(selectedPairs)
      .filter(([, on]) => on)
      .map(([k]) => {
        const [transactionId, documentId] = k.split("::");
        return { transactionId, documentId };
      });
    setBusy(true);
    try {
      for (const it of items) {
        try {
          await api.matchingConfirm(it.transactionId, it.documentId, privatePercent);
        } catch {
          /* exceptions shown via preview */
        }
      }
      setSelectedPairs({});
      setPreview(null);
      await reload();
    } finally {
      setBusy(false);
    }
  }

  if (loading) {
    return (
      <div className="bw-workbench">
        <LoadingState label="Abgleich wird geladen …" />
        <style>{fixtureStateCss}</style>
      </div>
    );
  }
  if (error && !snap) {
    return (
      <div className="bw-workbench">
        <ErrorState title="Abgleich nicht verfügbar" body={error} />
        <style>{fixtureStateCss}</style>
      </div>
    );
  }
  if (!snap || snap.transactions.length === 0) {
    return (
      <div className="bw-workbench">
        <EmptyState
          title="Keine Transaktionen"
          body="Zuerst unter Konten die Bank-Fixtures importieren, dann hier Vorschläge neu berechnen."
        />
        <div className="bw-actions">
          <button type="button" className="bw-btn bw-btn-accent" disabled={busy} onClick={() => void rebuild()}>
            Vorschläge neu berechnen
          </button>
        </div>
        <style>{workbenchCss}</style>
        <style>{fixtureStateCss}</style>
      </div>
    );
  }

  const selCount = Object.values(selectedPairs).filter(Boolean).length;

  return (
    <div className="bw-workbench" aria-label="Abgleich-Arbeitsansicht">
      <p className="bw-workbench-note">{snap.note}</p>
      {error ? <p className="bw-warn">{error}</p> : null}
      <div className="bw-batch-bar">
        <label className="bw-check">
          Privatanteil %
          <input
            type="number"
            min={0}
            max={100}
            value={privatePercent}
            onChange={(e) =>
              setPrivatePercent(Math.min(100, Math.max(0, Number(e.target.value) || 0)))
            }
            style={{ width: "4rem", marginLeft: "0.5rem" }}
          />
        </label>
        <button type="button" className="bw-btn" disabled={busy} onClick={() => void rebuild()}>
          Vorschläge neu berechnen
        </button>
      </div>

      <div className="bw-batch-bar" role="toolbar" aria-label="Stapelaktionen">
        <span className="bw-list-meta">{selCount} ausgewählt</span>
        <button type="button" className="bw-btn" disabled={selCount === 0 || busy} onClick={() => void runPreview()}>
          Vorschau
        </button>
        <button
          type="button"
          className="bw-btn bw-btn-accent"
          disabled={selCount === 0 || busy}
          onClick={() => void confirmSelected()}
        >
          Auswahl bestätigen
        </button>
        {preview ? (
          <span className="bw-list-meta">
            bestätigbar {preview.confirmable_count ?? preview.confirmableCount ?? 0} · Δ{" "}
            {formatMoney(
              preview.sum_allocated_minor ?? preview.sumAllocatedMinor ?? 0,
              "EUR",
            )}{" "}
            · Ausnahmen {preview.exception_count ?? preview.exceptionCount ?? 0}
          </span>
        ) : null}
      </div>
      {preview?.exceptions?.length ? (
        <ul className="bw-exception-list">
          {preview.exceptions.map((ex) => (
            <li key={ex}>{ex}</li>
          ))}
        </ul>
      ) : null}

      <div className="bw-workbench-grid">
        <section aria-label="Transaktionen">
          <h3>Transaktionen</h3>
          <ul className="bw-list">
            {snap.transactions.map((tx) => {
              const sug = snap.suggestions.find((s) => s.transactionId === tx.id);
              const pairKey = sug ? `${sug.transactionId}::${sug.documentId}` : "";
              return (
                <li key={tx.id}>
                  <button
                    type="button"
                    className={`bw-list-item${selectedTxId === tx.id ? " is-active" : ""}`}
                    onClick={() => setSelectedTxId(tx.id)}
                  >
                    <span className="bw-list-title">{tx.counterparty}</span>
                    <span className="bw-list-meta">
                      {tx.bookingDate} · {formatMoney(tx.amountMinor, tx.currency)}
                    </span>
                    <span className="bw-list-meta">{tx.description}</span>
                    <span className="bw-list-meta">
                      Rest {formatMoney(tx.openRemainderMinor, tx.currency)} · allokiert{" "}
                      {formatMoney(tx.allocatedMinor, tx.currency)}
                    </span>
                  </button>
                  {sug && !sug.decision ? (
                    <label className="bw-check">
                      <input
                        type="checkbox"
                        checked={Boolean(selectedPairs[pairKey])}
                        onChange={(e) =>
                          setSelectedPairs((prev) => ({ ...prev, [pairKey]: e.target.checked }))
                        }
                      />
                      Für Stapel
                    </label>
                  ) : null}
                </li>
              );
            })}
          </ul>
        </section>

        <section aria-label="Beleg">
          <h3>Beleg</h3>
          {linkedDoc ? (
            <article className="bw-doc">
              <p className="bw-list-title">{linkedDoc.filename}</p>
              <p className="bw-list-meta">
                Ref {linkedDoc.invoiceRef} ·{" "}
                {formatMoney(linkedDoc.amountMinor, linkedDoc.currency)}
              </p>
              <pre className="bw-excerpt">{linkedDoc.excerpt}</pre>
            </article>
          ) : (
            <EmptyState
              title="Kein Vorschlagsbeleg"
              body="Für diese Transaktion liegt kein automatischer Treffer vor."
            />
          )}
        </section>

        <section aria-label="Prüfung">
          <h3>Prüfung / Allocation</h3>
          {selectedTx ? (
            <div className="bw-review">
              <p>
                <strong>Status:</strong> {selectedTx.status}
              </p>
              <p className="bw-list-meta">
                Offener Rest: {formatMoney(selectedTx.openRemainderMinor, selectedTx.currency)}
              </p>
              {suggestion ? (
                <>
                  <p>
                    <strong>Vorschlag:</strong> {suggestion.confidence}
                    {suggestion.decision ? ` · ${suggestion.decision}` : ""}
                  </p>
                  <p className="bw-list-meta">{suggestion.reason}</p>
                  {!suggestion.decision ? (
                    <div className="bw-actions">
                      <button
                        type="button"
                        className="bw-btn bw-btn-accent"
                        disabled={busy || selectedTx.openRemainderMinor <= 0}
                        onClick={() => void confirm()}
                      >
                        Bestätigen (PaymentAllocation)
                      </button>
                      <button
                        type="button"
                        className="bw-btn"
                        disabled={busy}
                        onClick={() => void reject()}
                      >
                        Ablehnen
                      </button>
                    </div>
                  ) : (
                    <p className="bw-ok">Entscheidung: {suggestion.decision}</p>
                  )}
                </>
              ) : (
                <p className="bw-warn">
                  Kein automatischer Treffer — manuelle Zuordnung oder fehlender Beleg.
                </p>
              )}
            </div>
          ) : (
            <EmptyState title="Keine Transaktion gewählt" body="Bitte links eine Zeile auswählen." />
          )}
        </section>
      </div>
      <style>{workbenchCss}</style>
      <style>{fixtureStateCss}</style>
    </div>
  );
}

const workbenchCss = `
.bw-workbench-note {
  margin: 0 0 var(--bw-space-4);
  color: var(--bw-color-text-secondary);
  font-size: 0.9rem;
}
.bw-batch-bar {
  display: flex;
  flex-wrap: wrap;
  gap: var(--bw-space-2);
  align-items: center;
  margin-bottom: var(--bw-space-4);
  padding: var(--bw-space-3);
  background: var(--color-surface-bright);
  border: 1px solid var(--bw-color-border-subtle);
  border-radius: var(--bw-radius-sm);
}
.bw-exception-list {
  margin: 0 0 var(--bw-space-4);
  padding-left: 1.2rem;
  color: var(--bw-color-status-warning);
  font-size: 0.9rem;
}
.bw-workbench-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: var(--bw-space-4);
}
.bw-workbench-grid h3 {
  margin: 0 0 var(--bw-space-2);
  font-size: 0.95rem;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  color: var(--bw-color-text-secondary);
}
.bw-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: var(--bw-space-2);
}
.bw-list-item {
  width: 100%;
  text-align: left;
  border: 1px solid var(--bw-color-border-subtle);
  background: var(--color-surface-bright);
  border-radius: var(--bw-radius-sm);
  padding: var(--bw-space-3);
  cursor: pointer;
  display: grid;
  gap: 0.15rem;
}
.bw-list-item.is-active {
  border-color: var(--bw-color-accent);
  background: var(--bw-color-surface-selected);
  box-shadow: inset 3px 0 0 var(--color-accent);
}
.bw-list-title { font-weight: 600; }
.bw-list-meta { color: var(--bw-color-text-secondary); font-size: 0.85rem; }
.bw-excerpt {
  white-space: pre-wrap;
  font-family: var(--bw-font-mono);
  font-size: 0.8rem;
  background: var(--color-surface-bright);
  padding: var(--bw-space-3);
  border-radius: var(--bw-radius-sm);
}
.bw-empty, .bw-warn { color: var(--bw-color-status-warning); }
.bw-ok { color: var(--bw-color-status-success); }
.bw-check {
  display: flex;
  gap: var(--bw-space-2);
  align-items: center;
  margin-top: var(--bw-space-1);
  font-size: 0.85rem;
  color: var(--bw-color-text-secondary);
}
.bw-actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--bw-space-2);
  margin-top: var(--bw-space-3);
}
.bw-btn {
  border: 1px solid var(--bw-color-border-subtle);
  background: var(--color-surface-bright);
  color: var(--bw-color-text-primary);
  border-radius: var(--bw-radius-pill);
  padding: var(--bw-space-2) var(--bw-space-3);
  cursor: pointer;
  font-weight: 500;
}
.bw-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.bw-btn:hover:not(:disabled) {
  background: var(--color-surface-raised);
}
.bw-btn-accent {
  background: var(--bw-color-accent);
  border-color: var(--bw-color-accent);
  color: var(--bw-color-on-accent);
}
.bw-btn-accent:hover:not(:disabled) {
  background: var(--bw-color-accent-hover);
  border-color: var(--bw-color-accent-hover);
}
@media (max-width: 960px) {
  .bw-workbench-grid { grid-template-columns: 1fr; }
}
`;
