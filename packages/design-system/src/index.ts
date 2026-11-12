export const NAV_ITEMS = [
  { id: "overview", label: "Übersicht" },
  { id: "inbox", label: "Eingang" },
  { id: "accounts", label: "Konten" },
  { id: "matching", label: "Abgleich" },
  { id: "missing", label: "Fehlende Belege" },
  { id: "archive", label: "Archiv/Suche" },
  { id: "accounting", label: "Buchhaltung" },
  { id: "assets", label: "Anlagen" },
  { id: "reports", label: "Berichte" },
  { id: "handoff", label: "Übergaben" },
  { id: "automations", label: "Automationen/Connectoren" },
  { id: "settings", label: "Einstellungen" },
] as const;

export type NavId = (typeof NAV_ITEMS)[number]["id"];
