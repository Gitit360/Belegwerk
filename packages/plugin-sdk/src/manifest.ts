export const SDK_VERSION = "1" as const;

export type PluginKind =
  | "bank-import"
  | "invoice-import"
  | "document-source"
  | "export"
  | "portal"
  | "mail"
  | "ocr"
  | "ai-provider"
  | "reporting"
  | "delivery"
  | "tax-rules";

export type PluginPermissions = {
  networkDomains: string[];
  filesystem: string[];
  credentialScopes: string[];
  maxRuntimeMs?: number;
  maxMemoryMb?: number;
};

export type PluginManifest = {
  id: string;
  version: string;
  sdkVersion: typeof SDK_VERSION | string;
  kind: PluginKind;
  displayName: string;
  description?: string;
  capabilities: string[];
  permissions: PluginPermissions;
  entrypoint: string;
  development?: boolean;
  signature?: string | null;
};
