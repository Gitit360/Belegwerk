import type { PluginManifest } from "@belegwerk/plugin-sdk";

export const manifest: PluginManifest = {
  id: "belegwerk.example.hello-document-source",
  version: "0.0.1",
  sdkVersion: "1",
  kind: "document-source",
  displayName: "Hello Document Source",
  description: "Synthetisches Beispielplugin — keine echten Belege",
  capabilities: ["documents.list", "documents.download"],
  permissions: {
    networkDomains: [],
    filesystem: [],
    credentialScopes: [],
    maxRuntimeMs: 60_000,
    maxMemoryMb: 256,
  },
  entrypoint: "stub",
  development: true,
};

/** Lists synthetic document refs only. */
export function listSyntheticDocuments() {
  return [
    {
      id: "synth-doc-001",
      filename: "synth-invoice.txt",
      note: "synthetic fixture — not a real Beleg",
    },
  ];
}
