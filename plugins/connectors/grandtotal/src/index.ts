import type { PluginManifest } from "@belegwerk/plugin-sdk";

export const manifest: PluginManifest = {
  id: "belegwerk.connector.grandtotal",
  version: "0.0.1",
  sdkVersion: "1",
  kind: "invoice-import",
  displayName: "GrandTotal 10",
  description: "File-based invoice import stub (D-02). No proprietary DB writes.",
  capabilities: ["documents.list", "documents.download"],
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

export type GrandTotalInvoiceStub = {
  invoiceNumber: string;
  issueDate: string;
  amountMinor: number;
  currency: string;
  customer: string;
  source: "synthetic-fixture";
};

/** Parse a future GT10 export file — stub returns synthetic data. */
export function parseExportFile(_path: string): GrandTotalInvoiceStub[] {
  return [
    {
      invoiceNumber: "GT-SYNTH-2001",
      issueDate: "2026-01-10",
      amountMinor: 119_000,
      currency: "EUR",
      customer: "Synthetic Client AG",
      source: "synthetic-fixture",
    },
  ];
}

export function writeProprietaryDb(): never {
  throw new Error("GrandTotal proprietary DB writes are forbidden (D-02)");
}
