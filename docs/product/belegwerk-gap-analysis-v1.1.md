# Belegwerk – Gap-Analyse v1.1

Stand: 12. November 2026 (Europe/Berlin)  
Quelle: `docs/localledger-product-description-source.md` (v1.0 LocalLedger)  
Ziel: `docs/belegwerk-product-description-v1.1.md`

---

## 1. Zweck

Strukturierte Erfassung aller Lücken, „noch prüfen“-Stellen und offenen Fragen aus der Quelle v1.0 — mit Entscheidung, Begründung und Status (entschieden vs. bewusst zurückgestellt).

---

## 2. Gaps und Entscheidungen

| ID | Gap in v1.0 | Entscheidung v1.1 | Status |
|---|---|---|---|
| G-01 | Arbeitstitel LocalLedger; Marke ungeprüft | Produktname **Belegwerk**; LocalLedger einmal als früherer Arbeitstitel | Entschieden (D-01) |
| G-02 | GrandTotal Version 9/10 noch prüfen | Ziel: **GrandTotal 10**, dateibasierter Adapter; Versionsprobe bei Setup; keine proprietäre DB | Entschieden (D-02) |
| G-03 | Bank X Professional Schnittstelle offen | Phase-2 nach MoneyMoney; Kern nur Adaptervertrag | Entschieden (D-03) |
| G-04 | Voranmeldung monatlich oder quartalsweise | Default **monatlich**; Wechsel via versioniertes `ReportingSchedule` | Entschieden (D-04) |
| G-05 | Dauerfristverlängerung „erst nach Nutzerbestätigung“ | Default **aus** bis Bestätigung in Settings | Entschieden (D-05) |
| G-06 | EU-Kunden „künftig möglich“ vs. Fallmatrix | **EU-B2B-Verkäufe außerhalb v1**; US-SaaS/RC/FX in Scope | Entschieden (D-06) |
| G-07 | Mobilfunkanbieter „noch eindeutig zu benennen“ | Placeholder-Slot **`mobile-de`**, kein Fake-Brand | Entschieden (D-07) |
| G-08 | RPO/RTO „mit Nutzer festlegen“ | RPO ≤ 24 h; RTO ≤ 4 h (Messziele) | Entschieden (D-08) |
| G-09 | Lizenz Produkt/SDK offen | App proprietär/privat; SDK-Text Apache-2.0-orientiert reserviert; finale Lizenz deferred | Entschieden/teilweise deferred (D-09) |
| G-10 | Stack „vorläufig“, SwiftUI-Alternative | **Tauri 2 + React + TypeScript + Rust**; SwiftUI nur per späterem ADR | Entschieden (D-10) |
| G-11 | SQLite-Verschlüsselung „konkret auswählen“ | At-rest Verschlüsselung (SQLCipher o.ä.) + Keychain; Final in M01 ADR | Engineering-Default (D-20) |
| G-12 | Originalspeicher ohne Detailformat | Content-addressierter Dateispeicher + SQLite-Metadaten | Entschieden (D-11) |
| G-13 | Codex „getrennt prüfen“ | Dev-only optional; keine Prod-Runtime-Abhängigkeit | Entschieden (D-12) |
| G-14 | Steuerboard „später“ | Post-M15; nicht M00–M14 | Entschieden (D-13) |
| G-15 | App-Locale/Währung nicht explizit | UI Deutsch, `de-DE`, EUR Default, Mehrwährung | Entschieden (D-14) |
| G-16 | Repo-Name / Alignment | Spezifikation bezieht sich auf `Gitit360/Belegwerk` | Entschieden (D-15) |
| G-17 | Proxy-Beleg „vorläufige Definition“ | Definition aus Quelle als verbindliche Arbeitsdefinition; Eigenbelege blockiert bis Regel | Entschieden (D-16) |
| G-18 | Meta-Nodes / Graphwunsch | Relational; Graph-UI deferred bis Nutzerbestätigung | Entschieden (D-17) |
| G-19 | Wer prüft Regelpakete fachlich? | Nutzer als primärer Fachprüfer; Parallelperiode Pflicht vor Prod | Entschieden (D-18) |
| G-20 | Aktuelles Steuerjahr / Wechselhistorie | Setup schlägt aktuelles Kalenderjahr vor; Nutzer bestätigt | Entschieden (D-19) |
| G-21 | Bank-/Kreditkartenkonten-IDs | Quellen geplant (GLS, VB, DB/M&M); IDs beim Setup, nicht hartcodiert | Entschieden (Prozess) |
| G-22 | Apple-Mail-Schnittstelle ungeprüft | M10 prüft reale Capabilities; IMAP-Fallback spezifiziert | Entschieden (Prozess) |
| G-23 | Anlagenbestand / spezielle Vorfälle | Ungeprüfte Fälle blockiert; Regelversionierung | Entschieden (Prozess) |
| G-24 | Backupziele (Ort/Medium) | RPO/RTO gesetzt; konkrete Ziele (Disk/Time Machine/…) Nutzer bestätigt vor Prod | Teilweise deferred |
| G-25 | Markenrechte „Belegwerk“ | Engineering-Name gesetzt; rechtliche Markenprüfung außerhalb Spec-Pakets | Bewusst deferred |
| G-26 | ERiC-/ELSTER-Entwicklerregistrierung | Modul M15+; Voraussetzungen vor Implementierung verifizieren | Bewusst deferred (Modul) |
| G-27 | Konkrete GrandTotal-Exportprofil-Liste | Adapter-Setup mit Probe; Profilmatrix in M09 | Bewusst deferred (Modul) |
| G-28 | Konkrete Bank-X-API/Export | Interface TBD im Adaptermodul (M15) | Bewusst deferred (Modul) |

---

## 3. Bewusst zurückgestellt (mit Grund)

| Thema | Grund | Wann wieder aufnehmen |
|---|---|---|
| Finale Open-Source-/Kommerzlizenz | Nutzerentscheid; SDK-Dokumentation bleibt versioniert | Vor öffentlicher SDK-Freigabe |
| Marken-/Namensrechte Belegwerk | Rechtliche Prüfung, kein Engineering-Blocker für Spec | Vor öffentlichem Marketing/Distribution |
| Konkrete Backup-Medien/Orte | Hängt von Nutzerhardware und Ops-Präferenz ab | Vor Produktivbetrieb / M14 |
| GrandTotal exakte Exportfelder | Erfordert laufende GT-10-Installation und Probe | M09 |
| Bank X konkrete Schnittstelle | Phase-2; MoneyMoney zuerst | M15 |
| EU-B2B-Fallregeln | Außerhalb v1-Matrix; Steuerrecht verifizieren | Nach v1-Stabilität / M15 |
| Visuelle Node-Graph-UI | UX-Entscheidung Nutzer; Listen reichen | Nach Nutzerbestätigung |
| ERiC Direktübermittlung | Externe Registrierung + Plattformcheck | M15 |
| Steuerboard-Anbindung | Nicht kritisch für Kernzuverlässigkeit | Post-M15 |
| Produktiv-Steuerwerte (Sätze, GWG-Grenzen je Jahr) | Keine erfundenen Rechtsgewissheiten | Vor M11 mit Quellenprüfung |

---

## 4. Steuer-Sicherheit (Tax-Safety)

Wo steuerliche Fakten Verifikation brauchen, entscheidet v1.1 **Prozess/Default für Engineering**, nicht „verifiziertes Recht“:

- Falltypen ohne freigegebene Regelversion → **blockiert + dokumentiert**.
- Regelupdates → neue Version; historische Festschreibungen unverändert.
- Formularmappings → gegen aktuelle Quellen vor M12 prüfen.
- Keine GoBD-/Rechtssicherheits-Claims ohne fachliche Prüfung.

---

## 5. Was v1.1 nicht „umsetzt“

„Vollständig umsetzen“ bezieht sich auf das **Spezifikationspaket** (§0.1), nicht auf die Mac-Anwendung. Produktcode bleibt bis Nutzerfreigabe von M00 untersagt.

---

*Ende Gap-Analyse v1.1.*
