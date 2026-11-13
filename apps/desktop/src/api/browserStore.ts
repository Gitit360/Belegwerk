/** In-memory browser/dev fallback mirroring Tauri workflow commands. */

import { MATCHING_WORKBENCH } from "../fixtures/matchingWorkbench";

export type TaxProfile = {
  vat_method: "ist" | "soll";
  reporting_cadence: "monthly" | "quarterly";
  dauerfrist_extension: boolean;
  tax_year: number;
};

type Alloc = {
  id: string;
  economicTransactionId: string;
  documentId: string;
  allocatedMinor: number;
  currency: string;
  createdAt: string;
};

type Decision = "confirmed" | "rejected";

type Batch = {
  id: string;
  sourceLabel: string;
  importedAt: string;
  note?: string;
  documentCount: number;
};

type Doc = {
  id: string;
  contentHash: string;
  mimeType: string;
  originalFilename?: string;
  byteSize: number;
  importBatchId: string;
  importedAt: string;
  isPlaceholder: boolean;
  text?: string;
};

type CaseRow = {
  id: string;
  note: string;
  escalationLevel: number;
  status: string;
  createdAt: string;
  economicTransactionId?: string;
};

type JournalRow = {
  id: string;
  status: string;
  description?: string;
  blockedUnsupported: boolean;
  blockReason?: string;
  rulePackId?: string;
  rulePackVersion?: string;
  rulePackStatus?: string;
  lines: Array<{
    account_code: string;
    debit_minor: number;
    credit_minor: number;
    currency: string;
  }>;
};

const g = globalThis as unknown as {
  __bwStore?: {
    decisions: Record<string, Decision>;
    allocations: Alloc[];
    batches: Batch[];
    documents: Doc[];
    cases: CaseRow[];
    journals: JournalRow[];
    tax: TaxProfile;
    bookingApprovals: Set<string>;
  };
};

function store() {
  if (!g.__bwStore) {
    g.__bwStore = {
      decisions: {},
      allocations: [],
      batches: [],
      documents: [],
      cases: [
        {
          id: crypto.randomUUID(),
          note: "Beleg zu mm-synth-001 fehlt (synthetisch)",
          escalationLevel: 1,
          status: "open",
          createdAt: new Date().toISOString(),
        },
      ],
      journals: [
        {
          id: "j-synth-ok",
          status: "draft",
          description: "synthetic expense draft",
          blockedUnsupported: false,
          rulePackId: "core",
          rulePackVersion: "0.0.0-test",
          rulePackStatus: "reviewed",
          lines: [
            { account_code: "4950", debit_minor: 2499, credit_minor: 0, currency: "EUR" },
            { account_code: "1200", debit_minor: 0, credit_minor: 2499, currency: "EUR" },
          ],
        },
        {
          id: "j-synth-eu",
          status: "blocked",
          blockedUnsupported: true,
          blockReason: "EU-B2B sale outside v1 matrix (D-06) — keine erfundenen Steuerregeln",
          lines: [],
        },
      ],
      tax: {
        vat_method: "ist",
        reporting_cadence: "monthly",
        dauerfrist_extension: false,
        tax_year: 2026,
      },
      bookingApprovals: new Set(),
    };
  }
  return g.__bwStore;
}

function key(tx: string, doc: string) {
  return `${tx}::${doc}`;
}

export const browserApi = {
  async matchingSnapshot() {
    const s = store();
    const transactions = MATCHING_WORKBENCH.transactions.map((t) => {
      const allocated = s.allocations
        .filter((a) => a.economicTransactionId === t.id)
        .reduce((n, a) => n + a.allocatedMinor, 0);
      return {
        id: t.id,
        bookingDate: t.bookingDate,
        amountMinor: t.amountMinor,
        currency: t.currency,
        description: t.description,
        counterparty: t.counterparty,
        status: t.status,
        allocatedMinor: allocated,
        openRemainderMinor: Math.max(Math.abs(t.amountMinor) - allocated, 0),
      };
    });
    return {
      note: MATCHING_WORKBENCH.note,
      transactions,
      documents: MATCHING_WORKBENCH.documents.map((d) => ({
        id: d.id,
        filename: d.filename,
        invoiceRef: d.invoiceRef,
        amountMinor: d.amountMinor,
        currency: d.currency,
        excerpt: d.excerpt,
      })),
      suggestions: MATCHING_WORKBENCH.suggestions.map((sg) => ({
        transactionId: sg.transactionId,
        documentId: sg.documentId,
        confidence: sg.confidence,
        reason: sg.reason,
        decision: s.decisions[key(sg.transactionId, sg.documentId)] ?? null,
      })),
      allocations: s.allocations,
    };
  },

  async matchingConfirm(transactionId: string, documentId: string) {
    const s = store();
    const tx = MATCHING_WORKBENCH.transactions.find((t) => t.id === transactionId);
    if (!tx) throw new Error("unbekannte Transaktion");
    const allocated = s.allocations
      .filter((a) => a.economicTransactionId === transactionId)
      .reduce((n, a) => n + a.allocatedMinor, 0);
    const rem = Math.max(Math.abs(tx.amountMinor) - allocated, 0);
    if (rem <= 0) throw new Error("kein offener Rest");
    s.decisions[key(transactionId, documentId)] = "confirmed";
    const alloc: Alloc = {
      id: crypto.randomUUID(),
      economicTransactionId: transactionId,
      documentId,
      allocatedMinor: rem,
      currency: tx.currency,
      createdAt: new Date().toISOString(),
    };
    s.allocations.push(alloc);
    return alloc;
  },

  async matchingReject(transactionId: string, documentId: string) {
    store().decisions[key(transactionId, documentId)] = "rejected";
    return { decision: "rejected" };
  },

  async matchingBatchPreview(items: Array<{ transactionId: string; documentId: string }>) {
    const snap = await browserApi.matchingSnapshot();
    const exceptions: string[] = [];
    let confirmable = 0;
    let sumAllocated = 0;
    let sumRem = 0;
    for (const it of items) {
      const tx = snap.transactions.find((t) => t.id === it.transactionId);
      const dec = snap.suggestions.find(
        (s) => s.transactionId === it.transactionId && s.documentId === it.documentId,
      )?.decision;
      if (!tx) {
        exceptions.push(`unbekannt: ${it.transactionId}`);
        continue;
      }
      if (dec === "rejected" || dec === "confirmed") {
        exceptions.push(`bereits entschieden: ${it.transactionId}`);
        continue;
      }
      if (tx.openRemainderMinor <= 0) {
        exceptions.push(`kein Rest: ${it.transactionId}`);
        continue;
      }
      confirmable += 1;
      sumAllocated += tx.openRemainderMinor;
      sumRem += 0;
    }
    return {
      selected_count: items.length,
      confirmable_count: confirmable,
      exception_count: exceptions.length,
      sum_allocated_minor: sumAllocated,
      sum_open_remainder_minor: sumRem,
      exceptions,
    };
  },

  async importFiles(sourceLabel: string, files: Array<{ filename: string; mimeType: string; textContent: string }>) {
    const s = store();
    const batch: Batch = {
      id: crypto.randomUUID(),
      sourceLabel,
      importedAt: new Date().toISOString(),
      documentCount: files.length,
    };
    s.batches.unshift(batch);
    for (const f of files) {
      s.documents.unshift({
        id: crypto.randomUUID(),
        contentHash: `browser-${f.filename}-${f.textContent.length}`,
        mimeType: f.mimeType,
        originalFilename: f.filename,
        byteSize: f.textContent.length,
        importBatchId: batch.id,
        importedAt: batch.importedAt,
        isPlaceholder: false,
        text: f.textContent,
      });
    }
    return { batchId: batch.id, documentCount: files.length };
  },

  async importListBatches() {
    return store().batches;
  },

  async importListDocuments(batchId?: string) {
    const docs = store().documents;
    return batchId ? docs.filter((d) => d.importBatchId === batchId) : docs;
  },

  async remindersList() {
    return store().cases.filter((c) => c.status !== "resolved" && c.status !== "dismissed");
  },

  async remindersEscalate(caseId: string) {
    const c = store().cases.find((x) => x.id === caseId);
    if (!c) throw new Error("Fall nicht gefunden");
    c.escalationLevel += 1;
    if (c.escalationLevel >= 2) c.status = "waiting_on_mail";
    return c.escalationLevel;
  },

  async remindersComplete(caseId: string) {
    const c = store().cases.find((x) => x.id === caseId);
    if (!c) throw new Error("Fall nicht gefunden");
    c.status = "resolved";
  },

  async journalList() {
    return store().journals;
  },

  async journalGrantApproval(draftId: string) {
    store().bookingApprovals.add(draftId);
  },

  async journalApprove(draftId: string) {
    const s = store();
    const row = s.journals.find((j) => j.id === draftId);
    if (!row) throw new Error("Entwurf nicht gefunden");
    if (row.blockedUnsupported) throw new Error(row.blockReason ?? "blocked");
    if (!s.bookingApprovals.has(draftId)) {
      throw new Error("permission: approval required for action");
    }
    row.status = "approved";
    return `authorized booking post for ${draftId}`;
  },

  async settingsGet(): Promise<TaxProfile> {
    return { ...store().tax };
  },

  async settingsSave(profile: TaxProfile) {
    store().tax = { ...profile };
    return store().tax;
  },
};
