import { NAV_ITEMS, type NavId } from "@belegwerk/design-system";

export { NAV_ITEMS, type NavId };

export const PLACEHOLDER_COPY: Record<NavId, { title: string; body: string }> = {
  overview: {
    title: "Übersicht",
    body: "Zusammenfassung folgt in späteren Modulen. Keine Buchhaltungsdaten in diesem Scaffold.",
  },
  inbox: {
    title: "Eingang",
    body: "Import-Warteschlange (M03+). Nur synthetische Fixtures in der Entwicklung.",
  },
  accounts: {
    title: "Konten",
    body: "MoneyMoney-Adapter ist Read-only (M04). Keine Bankzahlungen.",
  },
  matching: {
    title: "Abgleich",
    body: "Matching-/Split-Typen sind im Domain-Crate vorbereitet (M05).",
  },
  missing: {
    title: "Fehlende Belege",
    body: "MissingDocumentCase und Reminder-Stubs (M06).",
  },
  archive: {
    title: "Archiv/Suche",
    body: "Content-addressierter Originalspeicher + Metadaten in SQLite (M03/M07).",
  },
  accounting: {
    title: "Buchhaltung",
    body: "SKR03-orientierte Strukturen ohne erfundene Steuersätze. Ungeklärte Fälle bleiben blockiert (M11).",
  },
  assets: {
    title: "Anlagen",
    body: "Anlagenbestand mit Jahresbezug — Regelwerte erst nach Quellenprüfung.",
  },
  reports: {
    title: "Berichte",
    body: "Steuer- und Managementberichte getrennt (M12). Nur Entwurfs-Shells.",
  },
  handoff: {
    title: "Übergaben",
    body: "Exportpakete und Kanzlei-Übergaben (M13). Steuerboard Post-M15.",
  },
  automations: {
    title: "Automationen/Connectoren",
    body: "Plugin-Host und Connector-Stubs (M02/M10). Keine Live-Credentials.",
  },
  settings: {
    title: "Einstellungen",
    body: "Profil, Rechte, Backupziele. Dauerfristverlängerung default aus.",
  },
};
