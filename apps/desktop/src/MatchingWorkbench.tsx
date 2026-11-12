import { useMemo, useState } from "react";
import {
  MATCHING_WORKBENCH,
  formatMoney,
  type FixtureDocument,
  type FixtureTransaction,
} from "./fixtures/matchingWorkbench";

export function MatchingWorkbench() {
  const [selectedTxId, setSelectedTxId] = useState(
    MATCHING_WORKBENCH.transactions[0]?.id ?? "",
  );

  const selectedTx: FixtureTransaction | undefined = useMemo(
    () => MATCHING_WORKBENCH.transactions.find((t) => t.id === selectedTxId),
    [selectedTxId],
  );

  const suggestion = MATCHING_WORKBENCH.suggestions.find(
    (s) => s.transactionId === selectedTxId,
  );
  const linkedDoc: FixtureDocument | undefined = suggestion
    ? MATCHING_WORKBENCH.documents.find((d) => d.id === suggestion.documentId)
    : undefined;

  return (
    <div className="bw-workbench" aria-label="Abgleich-Arbeitsansicht">
      <p className="bw-workbench-note">{MATCHING_WORKBENCH.note}</p>
      <div className="bw-workbench-grid">
        <section aria-label="Transaktionen">
          <h3>Transaktionen</h3>
          <ul className="bw-list">
            {MATCHING_WORKBENCH.transactions.map((tx) => (
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
                </button>
              </li>
            ))}
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
            <p className="bw-empty">Kein Vorschlagsbeleg für diese Transaktion.</p>
          )}
        </section>

        <section aria-label="Prüfung">
          <h3>Prüfung / Entwurf</h3>
          {selectedTx ? (
            <div className="bw-review">
              <p>
                <strong>Status:</strong> {selectedTx.status}
              </p>
              {suggestion ? (
                <>
                  <p>
                    <strong>Vorschlag:</strong> {suggestion.confidence}
                  </p>
                  <p className="bw-list-meta">{suggestion.reason}</p>
                  <p className="bw-ok">Heuristik: Referenz + Betrag (Engine-Stub)</p>
                </>
              ) : (
                <p className="bw-warn">
                  Kein automatischer Treffer — manuelle Zuordnung oder fehlender Beleg.
                </p>
              )}
              <p className="bw-list-meta">
                Steuerliche Buchung bleibt blockiert, bis ein freigegebenes Regelpaket vorliegt.
              </p>
            </div>
          ) : (
            <p className="bw-empty">Keine Transaktion gewählt.</p>
          )}
        </section>
      </div>
      <style>{workbenchCss}</style>
    </div>
  );
}

const workbenchCss = `
.bw-workbench-note {
  margin: 0 0 var(--bw-space-4);
  color: var(--bw-color-text-secondary);
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
  background: var(--bw-color-surface-muted);
  border-radius: var(--bw-radius-sm);
  padding: var(--bw-space-3);
  cursor: pointer;
  display: grid;
  gap: 0.15rem;
}
.bw-list-item.is-active {
  border-color: var(--bw-color-accent);
  background: var(--bw-color-surface-selected);
}
.bw-list-title { font-weight: 600; }
.bw-list-meta { color: var(--bw-color-text-secondary); font-size: 0.85rem; }
.bw-excerpt {
  white-space: pre-wrap;
  font-family: var(--bw-font-mono);
  font-size: 0.8rem;
  background: var(--bw-color-surface-muted);
  padding: var(--bw-space-3);
  border-radius: var(--bw-radius-sm);
}
.bw-empty, .bw-warn { color: var(--bw-color-status-warning); }
.bw-ok { color: var(--bw-color-status-success); }
@media (max-width: 960px) {
  .bw-workbench-grid { grid-template-columns: 1fr; }
}
`;
