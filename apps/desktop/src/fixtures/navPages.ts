/** Synthetic fixture data for German nav pages — not productive Belege. */

export const ACCOUNTS_FIXTURE = {
  note: "Read-only MoneyMoney fixture replay — keine Live-AppleScript-Daten",
  accounts: [
    {
      id: "synth-gls-main",
      label: "GLS Hauptkonto (synthetisch)",
      provider: "moneymoney",
      currency: "EUR",
    },
  ],
  recent: [
    {
      ref: "mm-synth-001",
      bookingDate: "2026-01-15",
      amountMinor: -2499,
      counterparty: "Synthetic Cloud Tools LLC",
      purpose: "INV-SYNTH-1001 SaaS",
    },
  ],
};

export const MISSING_FIXTURE = {
  note: "MissingDocumentCase + DocumentPlaceholder (synthetisch)",
  cases: [
    {
      id: "case-synth-001",
      note: "Beleg zu mm-synth-001 fehlt",
      escalation: 1,
      placeholder: "DocumentPlaceholder: Vendor PDF ausstehend",
    },
  ],
};

export const ARCHIVE_FIXTURE = {
  note: "Content-addressierte Originale — nur synthetischer Text",
  documents: [
    {
      filename: "synth-invoice.txt",
      hashPrefix: "sha256:…",
      excerpt: "Invoice: INV-SYNTH-1001",
    },
  ],
};

export const REPORTS_FIXTURE = {
  note: "Management-Basis, kein Steuerformular",
  lines: [
    {
      label: "Synthetische Ausgaben",
      amountMinor: -2499,
      currency: "EUR",
      drillDown: "eco-synth-001 → mm-synth-001",
    },
    {
      label: "Synthetische Einnahmen GT",
      amountMinor: 119000,
      currency: "EUR",
      drillDown: "GT-SYNTH-2001",
    },
  ],
};

export const AUTOMATIONS_FIXTURE = {
  connectors: [
    { id: "moneymoney", label: "MoneyMoney", status: "fixture-replay" },
    { id: "grandtotal", label: "GrandTotal 10", status: "file-stub" },
    { id: "mobile-de", label: "Mobilfunk (Slot)", status: "unconfigured" },
  ],
  extensionsBlocked: ["Bank X", "Steuerboard", "ERiC", "EU-B2B"],
};

export function formatMoney(amountMinor: number, currency: string): string {
  const sign = amountMinor < 0 ? "−" : "";
  return `${sign}${(Math.abs(amountMinor) / 100).toFixed(2)} ${currency}`;
}
