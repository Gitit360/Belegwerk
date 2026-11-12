# Belegwerk – M00 Arbeitsplan v1.1

Stand: 12. November 2026 (Europe/Berlin)  
Modul: **M00 – Discovery und Spezifikation**  
Repo-Ziel: `Gitit360/Belegwerk`  
**Gate: Kein Produktanwendungscode vor ausdrücklicher Nutzerfreigabe dieses Plans / der M00-Abnahme.**

---

## 1. Ziel von M00

Die Spezifikation so verbindlich machen, dass M01 (Repository, Architekturgerüst, Designsystem) ohne Begriffs-/Grenzen-/Stack-Streit starten kann. M00 liefert **Dokumentation und Nachweise**, keine Buchhaltungs-Features.

---

## 2. Bereits im Context-Paket v1.1 erledigt (Spec-Seite)

Diese Dateien erfüllen den §0.1-Geist „vorbereiten“ im Project Store:

| Artefakt | Pfad |
|---|---|
| Product Description | `docs/belegwerk-product-description-v1.1.md` |
| Gap-Analyse | `docs/belegwerk-gap-analysis-v1.1.md` |
| Traceability | `docs/belegwerk-traceability-matrix-v1.1.md` |
| Decision Register | `docs/belegwerk-decision-register-v1.1.md` |
| Architekturvorschlag | `docs/belegwerk-architecture-proposal-v1.1.md` |
| Plugin-Contract | `docs/belegwerk-plugin-contract-v1.1.md` |
| Dieser Plan | `docs/belegwerk-m00-workplan-v1.1.md` |
| Index | `docs/README.md` |

**Nächster Schritt nach Nutzerfreigabe:** M00-Artefakte ins Git-Repo übernehmen und unten gelistete Repo-Dateien anlegen — weiterhin **ohne** `apps/`-Produktlogik.

---

## 3. Scope M00 (nach Freigabe, im Repo)

### In Scope

1. Repo-Inventory (existierende Dateien prüfen; leeres oder neues Repo dokumentieren).
2. Requirements mit IDs aus Traceability nach `docs/product/REQUIREMENTS.md` spiegeln.
3. Decision Register nach `docs/architecture/DECISIONS.md` spiegeln; ADR-Stubs:
   - ADR-0001 Stack Tauri2+React+TS+Rust+SQLite
   - ADR-0002 Persistenz SQLite + content-addressed store
   - ADR-0003 Plugin-Host isolation approach (Spike-Fragen)
4. Bedrohungsmodell-Skizze → `docs/security/SECURITY_MODEL.md` (aus Architektur §11 erweitern).
5. Quellenregister → `docs/product/SOURCES.md` (aus Product Description §30).
6. Plugin-Vertragsentwurf → `docs/plugins/PLUGIN_SPEC.md`.
7. `PROJECT_STATUS.md` initialisieren (Modulstatus M00 in review / waiting).
8. `OPEN_QUESTIONS.md` nur mit bewusst deferred Items aus Gap-Analyse §3.
9. Synthetische Fixture-Policy dokumentieren (keine echten Belege).

### Out of Scope (explizit)

- Keine Desktop-App, keine Crates-Implementierung der Domäne.
- Keine echten MoneyMoney-/GrandTotal-/ELSTER-Calls.
- Keine produktiven Credentials oder Belege.
- Kein M01-CI-Greenfield außer dokumentierte Vorbereitungsliste.

---

## 4. Geplante Dateien (Repo, nach Freigabe)

```text
PROJECT_STATUS.md
docs/product/REQUIREMENTS.md
docs/product/SOURCES.md
docs/product/belegwerk-product-description-v1.1.md   # Kopie/Spiegel
docs/architecture/DECISIONS.md
docs/architecture/adr/ADR-0001-stack.md
docs/architecture/adr/ADR-0002-persistence.md
docs/architecture/adr/ADR-0003-plugin-host.md
docs/security/SECURITY_MODEL.md
docs/plugins/PLUGIN_SPEC.md
docs/operations/BACKUP_RESTORE.md          # Stub mit RPO/RTO-Zielen
docs/acceptance/M00_ACCEPTANCE.md
OPEN_QUESTIONS.md
```

Keine `src/`-Produktmodule in M00.

---

## 5. Risiken

| Risiko | Auswirkung | Gegenmaßnahme |
|---|---|---|
| Nutzer lehnt Stack (Tauri) ab | M01-Umbau | Freigabe-Gate vor Code; Alternative nur per ADR |
| Repo `Gitit360/Belegwerk` noch nicht bereit | Verzögerung Spiegelung | Spec bleibt im Project Store gültig |
| Steuerdefaults werden als Rechtsgewissheit missverstanden | Falsche Buchungen später | Tax-Safety-Sprache; blockierte Fälle |
| Scope Creep „doch schon UI“ | Unabgenommener Code | Out-of-Scope-Liste + DoD |
| Externe API-Annahmen falsch | Adapter-Rework | Verify-before-implement in M04/M09/M10 |

---

## 6. Abnahmekriterien M00

- [ ] Nutzer bestätigt **Begriffe**: Belegwerk, Proxy=`DocumentPlaceholder`, Meta-Nodes relational.
- [ ] Nutzer bestätigt **Produktgrenzen** inkl. EU-B2B out of v1, keine Bankzahlung.
- [ ] Nutzer bestätigt **Rechtepolitik** (§4) und Plugin-Isolation.
- [ ] Nutzer bestätigt **Stack**: Tauri 2 + React + TypeScript + Rust + SQLite.
- [ ] Decision Register gelesen; Einwände dokumentiert oder akzeptiert.
- [ ] Traceability liegt vor und deckt §0–§31 ab.
- [ ] Offen: nur bewusst deferred Items in `OPEN_QUESTIONS.md`.
- [ ] Explizite schriftliche Freigabe: „M00 abgenommen; M01 darf starten“ **oder** „M00 Docs ins Repo, danach Stop“.
- [ ] **Kein** Produktanwendungscode merged/gepusht unter dem Vorwand M00.

---

## 7. Wait-for-Approval Gate

```text
┌──────────────────────────────────────────────┐
│  JETZT: Spec-Paket v1.1 im Project Store     │
│  STATUS: waiting for user approval           │
└──────────────────────┬───────────────────────┘
                       │ Nutzerfreigabe
                       ▼
┌──────────────────────────────────────────────┐
│  M00 Repo-Spiegelung (Docs only)             │
└──────────────────────┬───────────────────────┘
                       │ M00 Abnahmeprotokoll
                       ▼
┌──────────────────────────────────────────────┐
│  M01 darf Produktgerüst-Code beginnen        │
└──────────────────────────────────────────────┘
```

**Bis die Nutzerfreigabe vorliegt, ist jede Implementierung von `apps/`, Domänen-Crates mit Geschäftslogik oder Connectors untersagt.**

---

## 8. Vorschlag nächster Schritt nach Freigabe

1. M00-Docs ins Repo spiegeln.  
2. Abnahmeprotokoll `M00_ACCEPTANCE.md` vom Nutzer bestätigen lassen.  
3. M01-Workplan vorlegen (CI, Skeleton, Design Tokens, Migrationsgerüst) — erneut als begrenztes Paket.

---

## 9. Cursor-Abschlussformat (für spätere M00-Repo-Arbeit)

```text
Arbeitspaket: M00 Discovery/Spezifikation
Umgesetzt: ...
Geänderte Dateien: ...
Datenmodell/Verträge: nur Docs/Vertragsskizzen
Ausgeführte Tests und Ergebnisse: n/a (keine Produktivcode-Tests)
Nicht ausgeführte Prüfungen: externe API-Verifikation (bewusst M04+)
Risiken/offene Fragen: siehe OPEN_QUESTIONS.md
Anforderungserfüllung: REQ-MOD-000, REQ-P-009
Nächster vorgeschlagener Schritt: M01 nach Freigabe
Benötigte Freigabe: Nutzer — M00 abnehmen / M01 starten
```

---

*Ende M00-Arbeitsplan v1.1.*
