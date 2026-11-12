import type { SourceTransactionDto } from "@belegwerk/contracts";
import type { PluginManifest } from "@belegwerk/plugin-sdk";

export const manifest: PluginManifest = {
  id: "belegwerk.connector.moneymoney",
  version: "0.0.1",
  sdkVersion: "1",
  kind: "bank-import",
  displayName: "MoneyMoney",
  description: "Read-only bank import stub. Live AppleScript only on macOS (M04).",
  capabilities: ["bank.transactions.read"],
  permissions: {
    networkDomains: [],
    filesystem: [],
    credentialScopes: [],
    maxRuntimeMs: 120_000,
    maxMemoryMb: 512,
  },
  entrypoint: "stub",
  development: true,
};

/**
 * MoneyMoney live AppleScript is macOS-only.
 * On Linux CI this adapter returns synthetic fixtures only.
 */
export async function fetchTransactions(_opts?: {
  accountIds?: string[];
}): Promise<SourceTransactionDto[]> {
  // Live AppleScript path is macOS-only and not wired yet (M04).
  // Always return synthetic fixtures in this stub.
  void _opts;
  return loadSyntheticFixture();
}

export function loadSyntheticFixture(): SourceTransactionDto[] {
  return [
    {
      id: "00000000-0000-4000-8000-000000000001",
      providerId: "moneymoney",
      sourceAccountId: "synth-gls-main",
      sourceTransactionRef: "mm-synth-001",
      bookingDate: "2026-01-15",
      valueDate: "2026-01-15",
      amount: { amountMinor: -2499, currency: "EUR" },
      counterparty: "Synthetic Cloud Tools LLC",
      purpose: "INV-SYNTH-1001 SaaS",
      status: "imported",
      importedAt: "2026-01-15T10:00:00Z",
      rawRef: "tests/fixtures/synthetic/bank/mm-synth-001.json",
    },
  ];
}

/** Permanently unavailable — REQ-S-004 */
export function createPayment(): never {
  throw new Error("bank.payment.create is not implemented and must not be added");
}
