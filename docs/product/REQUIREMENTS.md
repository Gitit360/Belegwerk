# Belegwerk – Requirements (Repo-Spiegel)

Stand: 12. November 2026 (Europe/Berlin)  
Quelle: Traceability-Matrix v1.1 (`belegwerk-traceability-matrix-v1.1.md`)  
Status-Legende: `Spec` · `Gate` · `Deferred` · `Scaffold` (M01 Gerüst begonnen)

Dieses Dokument spiegelt die Requirement-IDs für die Arbeit im Git-Repo. Detailbeschreibung bleibt in der Product Description v1.1.

---

## Produkt & Prozess

| ID | Kurz | Modul | Status |
|---|---|---|---|
| REQ-P-001 | Modulare Arbeit; kein Big-Bang | M00–M15 | Spec |
| REQ-P-002 | Vor Implementierung Modul-Anforderungen lesen | alle | Spec |
| REQ-P-003 | Arbeitspaket-Beschreibung vor Änderungen | alle | Spec |
| REQ-P-004 | Tests ausführen und echte Ergebnisse protokollieren | alle | Spec |
| REQ-P-005 | Meilenstein-Abnahme vor nächstem Modul | alle | Spec |
| REQ-P-006 | Keine vermuteten Steuerregeln; Unklar → blockiert | M11+ | Spec |
| REQ-P-007 | Nur synthetische/anonymisierte Dev-Daten | alle | Spec |
| REQ-P-008 | Externe APIs vor Implementierung verifizieren | Adapter | Spec |
| REQ-P-009 | Kein Produktcode vor M00-Freigabe | M00 | Gate (Freigabe erteilt für Docs+Scaffold) |
| REQ-P-010 | Produktname Belegwerk; Docs DE / Code EN | M00–M01 | Spec |

## Vision & Grenzen

| ID | Kurz | Modul | Status |
|---|---|---|---|
| REQ-V-001 | Kernworkflow Import→Match→Buchung→Auswertung | M03–M12 | Spec |
| REQ-V-002 | KI assistiert, entscheidet nicht steuerlich | M08, M11 | Spec |
| REQ-B-001 | Zielumfang Archiv/Match/Buchhaltung/API/Export | M03–M13 | Spec |
| REQ-B-002 | Out-of-scope Lohn/Warenwirtschaft/ESt-gesamt | — | Spec |
| REQ-B-003 | EU-B2B-Verkäufe außerhalb v1 | M11, M15 | Spec |
| REQ-B-004 | Nicht unterstützte Fälle sichtbar, nie still mappen | M05, M11 | Spec |

## Sicherheit & Freigaben (Auszug)

| ID | Kurz | Modul | Status |
|---|---|---|---|
| REQ-S-001 | Permission Gate im Kern | M02 | Spec |
| REQ-S-004 | Keine Bankzahlungsfunktion | M02–M04 | Spec |
| REQ-S-008 | DB at-rest Encryption + Keychain | M01, M14 | Spec |
| REQ-S-009 | Keine Default-Telemetrie | M01 | Scaffold |

## UX / Architektur / Plugin (M01-relevant)

| ID | Kurz | Modul | Status |
|---|---|---|---|
| REQ-U-004 | Design Tokens vor Fachseiten | M01 | Scaffold |
| REQ-U-006 | UI-Sprache Deutsch / de-DE | M01 | Scaffold |
| REQ-AR-001 | Tauri2+React+TS+Rust+SQLite | M00, M01 | Scaffold |
| REQ-AR-002 | Schichten UI→App→Domain→Ports→Adapters | M01 | Scaffold |
| REQ-PL-001 | Versioniertes Plugin-Konzept | M02 | Scaffold (SDK-Typen) |
| REQ-DM-002 | UTC Zeitpunkte; Berlin Kalenderdaten | M01 | Spec |
| REQ-D-004 | Content-addressierter Originalspeicher | M03 | Scaffold (Pfad-Stub) |
| REQ-BU-002 | RPO ≤ 24 h; RTO ≤ 4 h | M14 | Spec (Doku) |
| REQ-MOD-000 | M00 Spec-Paket + Nutzerfreigabe | M00 | Gate |
| REQ-MOD-001 | M01 Gerüst | M01 | Scaffold |

Vollständige Matrix: `docs/product/belegwerk-traceability-matrix-v1.1.md`.

---

## Fixture-Policy (REQ-P-007)

- Erlaubt: synthetische / ausdrücklich anonymisierte Daten unter `tests/fixtures/synthetic/`.
- Verboten: echte Zugangsdaten, produktive Belege, Klartext-Secrets in Repo, Logs oder Prompts.
- Siehe auch `tests/fixtures/synthetic/README.md`.
