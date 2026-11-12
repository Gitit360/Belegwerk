/** Synthetic matching workbench fixture — not real bank/invoice data. */

export type FixtureTransaction = {
  id: string;
  bookingDate: string;
  amountMinor: number;
  currency: string;
  description: string;
  counterparty: string;
  status: string;
};

export type FixtureDocument = {
  id: string;
  filename: string;
  invoiceRef: string;
  amountMinor: number;
  currency: string;
  excerpt: string;
};

export type FixtureSuggestion = {
  transactionId: string;
  documentId: string;
  confidence: string;
  reason: string;
};

export const MATCHING_WORKBENCH = {
  note: "synthetic only — not real bank or invoice data",
  transactions: [
    {
      id: "eco-synth-001",
      bookingDate: "2026-01-15",
      amountMinor: -2499,
      currency: "EUR",
      description: "INV-SYNTH-1001 SaaS",
      counterparty: "Synthetic Cloud Tools LLC",
      status: "imported",
    },
    {
      id: "eco-synth-002",
      bookingDate: "2026-01-18",
      amountMinor: -8990,
      currency: "EUR",
      description: "Office supplies SYNTH",
      counterparty: "Synthetic Office GmbH",
      status: "imported",
    },
  ] satisfies FixtureTransaction[],
  documents: [
    {
      id: "doc-synth-001",
      filename: "synth-invoice.txt",
      invoiceRef: "INV-SYNTH-1001",
      amountMinor: 2499,
      currency: "EUR",
      excerpt:
        "SYNTHETIC INVOICE — NOT A REAL BELEG\nInvoice: INV-SYNTH-1001\nAmount: EUR 24.99",
    },
    {
      id: "doc-synth-002",
      filename: "synth-office.txt",
      invoiceRef: "OFF-900",
      amountMinor: 5000,
      currency: "EUR",
      excerpt: "SYNTHETIC — unrelated amount for low/no match demo",
    },
  ] satisfies FixtureDocument[],
  suggestions: [
    {
      transactionId: "eco-synth-001",
      documentId: "doc-synth-001",
      confidence: "exact",
      reason: "reference_match:INV-SYNTH-1001+amount_match:2499 EUR",
    },
  ] satisfies FixtureSuggestion[],
};

export function formatMoney(amountMinor: number, currency: string): string {
  const sign = amountMinor < 0 ? "−" : "";
  const abs = Math.abs(amountMinor);
  const major = (abs / 100).toFixed(2);
  return `${sign}${major} ${currency}`;
}
