# Belegwerk – Decision Register v1.1

Stand: 12. November 2026 (Europe/Berlin)  
Bezug: Product Description v1.1, Gap-Analyse v1.1  
Format je Entscheidung: Kontext → Optionen → Wahl → Konsequenzen → Status

---

## D-01 Produktname

**Kontext:** Quelle nutzte Arbeitstitel LocalLedger; Marke ungeprüft.  
**Optionen:** (A) LocalLedger beibehalten (B) Belegwerk (C) anderer Name später.  
**Wahl:** **Belegwerk**; LocalLedger einmal als früherer Arbeitstitel erwähnen.  
**Konsequenzen:** Alle IDs/Docs/Plugin-Präfixe auf `belegwerk.*` ausrichten; Markenprüfung separat.  
**Status:** Entschieden (Engineering); rechtliche Markenprüfung deferred.

---

## D-02 GrandTotal-Adapter

**Kontext:** Version 9/10 und Exportwege offen.  
**Optionen:** (A) GT9 zuerst (B) GT10 dateibasiert zuerst (C) direkte DB.  
**Wahl:** **GrandTotal 10, dateibasierter Export/Import**; Versionsprobe bei Setup; **keine proprietären DB-Writes**.  
**Konsequenzen:** M09 baut Profilmatrix; Fallback/Fehlermeldung wenn Version ≠ erwartet; Zahlungsstatus-Rückschreiben später.  
**Status:** Entschieden.

---

## D-03 Bank X Professional

**Kontext:** Schnittstelle ungeprüft.  
**Optionen:** (A) Parallel zu MoneyMoney (B) Phase-2 nach MoneyMoney (C) weglassen.  
**Wahl:** **Phase-2 nach MoneyMoney**; Kern nur Adaptervertrag; Interface TBD im Adaptermodul.  
**Konsequenzen:** Kein Blocker für M04–M14; Slot in Kompatibilitätsmatrix.  
**Status:** Entschieden (Timing); Interface deferred auf Modulzeitpunkt.

---

## D-04 USt-Voranmeldungsrhythmus

**Kontext:** Monatlich oder quartalsweise, zeitlich versioniert.  
**Optionen:** (A) Default monatlich (B) Default quartalsweise (C) erzwingen ohne Wechsel.  
**Wahl:** **Default monatliche Istversteuerung**; Wechsel auf Quartal über versioniertes `ReportingSchedule` mit Gültigkeitsdaten.  
**Konsequenzen:** Formulare folgen aktiver Schedule-Version; kein ad-hoc Umschalten pro Formular.  
**Status:** Entschieden (Engineering-Default; Nutzer bestätigt beim Setup).

---

## D-05 Dauerfristverlängerung

**Kontext:** Quelle: erst nach Nutzerbestätigung.  
**Optionen:** (A) aus (B) an (C) aus UI entfernen.  
**Wahl:** **Aus bis Nutzer in Settings bestätigt.**  
**Konsequenzen:** Fristenlogik und Reminder nutzen Flag; keine stillen Deadline-Verschiebungen.  
**Status:** Entschieden.

---

## D-06 EU-B2B-Verkäufe

**Kontext:** „EU-Kunden künftig möglich“ vs. Fallmatrix.  
**Optionen:** (A) in v1 (B) außerhalb v1 (C) teilweise.  
**Wahl:** **Explizit außerhalb v1-Fallmatrix**; US-SaaS / unterstützte Reverse-Charge / FX bleiben in Scope.  
**Konsequenzen:** EU-B2B-Fälle → blockiert + dokumentiert bis M15-Erweiterung.  
**Status:** Entschieden.

---

## D-07 Mobilfunk-Connector

**Kontext:** Anbieter nicht eindeutig.  
**Optionen:** (A) Fake-Brand erfinden (B) Slot `mobile-de` (C) weglassen.  
**Wahl:** Placeholder **`mobile-de`**; Nutzer konfiguriert später.  
**Konsequenzen:** Kein fingierter Markenname in Spec/UI.  
**Status:** Entschieden.

---

## D-08 RPO / RTO

**Kontext:** Quelle verlangte Nutzerfestlegung.  
**Optionen:** diverse Zielwerte.  
**Wahl:** **RPO ≤ 24 h** lokales Backup; **RTO ≤ 4 h** dokumentierte Restore auf gleicher Mac-Klasse — Messziele, kein SLA.  
**Konsequenzen:** Backup-Jobs und Doku müssen messbar sein; konkrete Medien deferred.  
**Status:** Entschieden (Ziele); Medien deferred.

---

## D-09 Lizenz

**Kontext:** Offen ≠ Open Source.  
**Optionen:** (A) alles proprietär (B) App + SDK OSS (C) App proprietär, SDK Apache-2.0-orientiert reserviert.  
**Wahl:** **App proprietär/privat vorerst**; Plugin-SDK-Vertragstext **Apache-2.0-orientiert reserviert**; finale Lizenz deferred.  
**Konsequenzen:** SDK bleibt dokumentiert/versioniert; Veröffentlichung unterliegt späterer Entscheidung.  
**Status:** Teilweise entschieden; finale Lizenz deferred.

---

## D-10 UI-/App-Stack

**Kontext:** Tauri vs. Electron vs. SwiftUI.  
**Optionen:** (A) Electron+React (B) Tauri 2+React+TS (C) SwiftUI native.  
**Wahl:** **Tauri 2 + React + TypeScript**; Domäne/Services in **Rust**. SwiftUI nur via späterem ADR.  
**Konsequenzen:** Repo-Struktur crates + packages; kleineres Binary, bessere Isolation; WebView-Grenzen dokumentieren.  
**Status:** Entschieden.

---

## D-11 Persistenz

**Kontext:** SQLite + Originale.  
**Optionen:** (A) alles in SQLite BLOBs (B) FS ohne Hash-Adressierung (C) SQLite + content-addressed FS.  
**Wahl:** **SQLite mit Migrationen** für strukturierte Daten; **content-addressierter Originalspeicher** auf dem Dateisystem.  
**Konsequenzen:** Dubletten über Hash; Originale unveränderlich referenziert.  
**Status:** Entschieden.

---

## D-12 Codex

**Kontext:** Separat prüfen.  
**Optionen:** (A) Prod-Runtime (B) Dev-only optional (C) ausschließen.  
**Wahl:** **Dev-only optional**; keine Produktionsabhängigkeit.  
**Konsequenzen:** Kern ohne Codex lauffähig; keine Token-Auslese/Proxys.  
**Status:** Entschieden.

---

## D-13 Steuerboard

**Kontext:** Späteres Plugin.  
**Optionen:** (A) in M13 (B) Post-M15 (C) Kernfeature.  
**Wahl:** **Post-M15**; nicht im kritischen Pfad M00–M14.  
**Konsequenzen:** Übergaben zuerst via generische Export-/Delivery-Ports.  
**Status:** Entschieden.

---

## D-14 Sprache, Locale, Währung

**Kontext:** Implizit Deutsch.  
**Optionen:** (A) DE only (B) i18n ab Tag 1.  
**Wahl:** **Deutsche UI**, Locale **`de-DE`**, Default-Währung **EUR**, Mehrwährungsfelder.  
**Konsequenzen:** Keine Parallel-i18n in v1; Code-IDs Englisch.  
**Status:** Entschieden.

---

## D-15 Repository-Alignment

**Kontext:** Repo ggf. noch in Anlage.  
**Optionen:** diverse Namen.  
**Wahl:** Spezifikation bezieht sich auf **`Gitit360/Belegwerk`**.  
**Konsequenzen:** Docs/Links nutzen diesen Namen; Existenz vor Push prüfen.  
**Status:** Entschieden (Referenz).

---

## D-16 Proxy-Beleg-Definition

**Kontext:** „Vorläufige Definition“.  
**Optionen:** (A) Placeholder wie beschrieben (B) Eigenbeleg-Äquivalent (C) warten.  
**Wahl:** **DocumentPlaceholder wie in Quelle** verbindlich für Engineering; Eigenbelege eigener blockierter Falltyp.  
**Konsequenzen:** Kein Vorsteuerabzug aus Proxy allein.  
**Status:** Entschieden (Arbeitsdefinition; Nutzer kann Fachdefinition später ändern).

---

## D-17 Meta-Nodes / Graph-UI

**Kontext:** Graphwunsch unklar.  
**Optionen:** (A) Graph DB + UI (B) relational + Listen (C) Graph-UI sofort.  
**Wahl:** **Relational in SQLite**; Listen/Beziehungen ohne Graph; Graph-UI erst nach Nutzerbestätigung (M15+).  
**Konsequenzen:** Keine Graph-DB-Abhängigkeit.  
**Status:** Entschieden.

---

## D-18 Fachprüfung Regelpakete

**Kontext:** „Wer prüft?“ offen.  
**Optionen:** (A) nur KI (B) Nutzer (C) Pflicht-Steuerberater-API.  
**Wahl:** **Nutzer als primärer Fachprüfer**; Parallelabstimmung mind. einer Periode vor Prod; externe Review empfohlen, nicht erzwungen.  
**Konsequenzen:** M11/M14-Gates bleiben menschlich.  
**Status:** Entschieden.

---

## D-19 Steuerjahr-Default

**Kontext:** Aktuelles Jahr / Historie offen.  
**Optionen:** (A) hart aktuelles Jahr (B) Setup-Vorschlag + Bestätigung (C) leeres Profil.  
**Wahl:** Setup schlägt **aktuelles Kalenderjahr** vor; Nutzer bestätigt oder setzt Historie.  
**Konsequenzen:** Keine stille Rückrechnung.  
**Status:** Entschieden.

---

## D-20 Verschlüsselung at rest

**Kontext:** „Verschlüsselung konkret auswählen“.  
**Optionen:** (A) unverschlüsselt + FileVault only (B) SQLCipher/äquivalent + Keychain (C) Full-disk App-Container only.  
**Wahl:** **DB at-rest verschlüsselt (SQLCipher oder gleichwertig)**; Schlüssel im Keychain; Finaltechnologie in M01 ADR.  
**Konsequenzen:** Backup muss Schlüsselstrategie dokumentieren; Restore-Tests schließen Entschlüsselung ein.  
**Status:** Engineering-Default; Technologie-Pin in M01 ADR.

---

## D-21 Zeitzonenmodell

**Kontext:** Quelle verlangt explizite Behandlung.  
**Optionen:** (A) alles lokal (B) UTC timestamps + Berlin calendar dates (C) nur Berlin.  
**Wahl:** **UTC für Zeitpunkte**; fachliche Kalenderdaten in **`Europe/Berlin`**, sofern nicht anders am Objekt.  
**Konsequenzen:** ReportingSchedule und Belegdaten konsistent.  
**Status:** Entschieden.

---

## D-22 Geldarithmetik

**Kontext:** Keine Floats.  
**Optionen:** (A) f64 (B) Decimal/Minor units in Integer (C) String-only.  
**Wahl:** **Minor-unit Integer + explizite Decimal-Strategie** in der Domäne; FX mit höherer Präzision, Quelle, Datum.  
**Konsequenzen:** Property-Tests auf Rundung; keine binären Floats für Geld.  
**Status:** Entschieden.

---

## Mapping Quelle §29 → Entscheidungen

| Offener Punkt Quelle §29 | Register |
|---|---|
| GrandTotal-Version/Export | D-02 |
| Proxy-Beleg-Bedeutung | D-16 |
| Meta-Nodes / Graph | D-17 |
| Mobilfunkanbieter | D-07 |
| Bank-/Kreditkartenkonten | Gap G-21 (Setup-Prozess) |
| Steuerjahr / Rhythmus / Dauerfrist | D-19, D-04, D-05 |
| Spezielle Vorfälle / Anlagen | Prozess: blockieren bis Regel |
| Fachprüfung | D-18 |
| Backupziele / RPO/RTO / Schlüssel | D-08, D-20; Medien deferred |
| Lizenz | D-09 |
| Codex | D-12 |
| Bank X / Apple Mail | D-03; Mail = M10-Prozess |

---

*Ende Decision Register v1.1.*
