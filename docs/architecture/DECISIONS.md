# Belegwerk – Decisions (Repo-Spiegel)

Stand: 12. November 2026 (Europe/Berlin)  
Vollständiges Register: `belegwerk-decision-register-v1.1.md`  
ADRs: `docs/architecture/adr/`

---

## Kurzüberblick

| ID | Thema | Wahl | Status |
|---|---|---|---|
| D-01 | Produktname | **Belegwerk** | Entschieden |
| D-02 | GrandTotal | GT10, dateibasiert; keine proprietäre DB | Entschieden |
| D-03 | Bank X Professional | Phase-2 nach MoneyMoney | Entschieden (Timing) |
| D-04 | USt-Voranmeldung | Default monatlich; versioniertes ReportingSchedule | Entschieden |
| D-05 | Dauerfristverlängerung | Aus bis Nutzerbestätigung | Entschieden |
| D-06 | EU-B2B-Verkäufe | Außerhalb v1 | Entschieden |
| D-07 | Mobilfunk-Connector | Slot `mobile-de`, kein Fake-Brand | Entschieden |
| D-08 | RPO/RTO | ≤24 h / ≤4 h Messziele | Entschieden |
| D-09 | Lizenz | App privat; SDK Apache-2.0-orientiert reserviert | Teilweise deferred |
| D-10 | Stack | **Tauri 2 + React + TypeScript + Rust + SQLite** | Entschieden → ADR-0001 |
| D-11 | Persistenz | SQLite + content-addressed FS | Entschieden → ADR-0002 |
| D-12 | Codex | Dev-only optional | Entschieden |
| D-13 | Steuerboard | Post-M15 | Entschieden |
| D-14 | Sprache/Locale | DE UI, de-DE, EUR Default | Entschieden |
| D-15 | Repo | `Gitit360/Belegwerk` | Entschieden |
| D-16 | Proxy-Beleg | DocumentPlaceholder wie spezifiziert | Entschieden |
| D-17 | Meta-Nodes | Relational; Graph-UI deferred | Entschieden |
| D-18 | Fachprüfung | Nutzer primär; Parallelperiode vor Prod | Entschieden |
| D-19 | Steuerjahr-Default | Setup-Vorschlag aktuelles Jahr | Entschieden |
| D-20 | Encryption at rest | SQLCipher o.ä. + Keychain; Pin M01 | Engineering-Default |
| D-21 | Zeitzonen | UTC timestamps; Europe/Berlin calendar dates | Entschieden |
| D-22 | Geldarithmetik | Minor-unit Integer + Decimal-Strategie | Entschieden |

---

## ADR-Stubs

| ADR | Titel | Bezug |
|---|---|---|
| [ADR-0001](./adr/ADR-0001-stack.md) | Stack Tauri 2 + React + TS + Rust + SQLite | D-10 |
| [ADR-0002](./adr/ADR-0002-persistence.md) | Persistenz SQLite + content-addressed store | D-11, D-20 |
| [ADR-0003](./adr/ADR-0003-plugin-host.md) | Plugin-Host Isolation (Spike-Fragen) | §20 / Plugin-Contract |
