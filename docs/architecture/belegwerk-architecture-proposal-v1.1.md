# Belegwerk – Architekturvorschlag v1.1

Stand: 12. November 2026 (Europe/Berlin)  
Status: Verbindlicher Engineering-Vorschlag für M00-Abnahme; Feindetails (SQLCipher-Pin, Host-Isolation) in M01 ADRs.  
Bezug: Product Description §19–§24; Decisions D-10, D-11, D-20, D-21, D-22.

---

## 1. Ziele

- **Local-first** Mac-App für einen Nutzer / ein Einzelunternehmen.
- Deterministischer Buchhaltungskern; KI und Adapter austauschbar.
- Plugin-fähig ab Tag 1 ohne Kernänderungen für neue Integrationen.
- Klare Sicherheitsgrenze: Permission Gate im Application-Layer.
- Keine Microservice-Landschaft; **modularer Monolith**.

---

## 2. Gewählter Stack (D-10)

| Schicht | Technologie | Begründung |
|---|---|---|
| Desktop Shell | **Tauri 2** | Kleineres Binary, OS-nahe Rechte, geringere Angriffsfläche als Electron |
| UI | **React + TypeScript** | Schnelle Fach-UI, Designsystem, Accessibility-Ökosystem |
| Domäne & Application | **Rust** (crates) | Invarianten, Geldarithmetik, Isolation, Testbarkeit |
| Lokale DB | **SQLite** + Migrationen | Single-file, robust, FTS |
| Originale | Content-addressiertes FS | Unveränderliche Referenzen, Dubletten |
| Secrets | macOS Keychain via Credential Broker | Keine Secrets in DB/Logs/Prompts |
| Jobs | Persistente Queue in SQLite | Crash-recoverable |
| Plugins | Isolierter Host-Prozess | Crashisolation, Capability Gate |

**Nicht gewählt:** Electron (größere Surface, schwereres Bundle) — nur falls Tauri-Hardblocker in M01 ADR; SwiftUI nur per späterem ADR, keine Parallel-UI.

---

## 3. Systemgrenzen

```text
┌─────────────────────────────────────────────────────────────┐
│  Surfaces: Desktop UI · CLI · Local REST · MCP               │
└────────────────────────────┬────────────────────────────────┘
                             │ typed commands / queries
┌────────────────────────────▼────────────────────────────────┐
│  Application Services                                        │
│  + Permission Gate + Approval Service + Job Orchestrator     │
└────────────────────────────┬────────────────────────────────┘
                             │ domain APIs (no raw SQL)
┌────────────────────────────▼────────────────────────────────┐
│  Domain                                                      │
│  Documents · Finance · Matching · Accounting · Reporting     │
└────────────────────────────┬────────────────────────────────┘
                             │ ports (traits / interfaces)
┌────────────────────────────▼────────────────────────────────┐
│  Adapters / Plugins                                          │
│  SQLite · BlobStore · MoneyMoney · GrandTotal · Mail · OCR   │
│  Ollama · OpenRouter · Portal Connectors · Export/Delivery   │
└─────────────────────────────────────────────────────────────┘
```

**Grenzregeln:**
1. UI/CLI/MCP enthalten **keine** Steuerberechnungslogik.
2. Plugins schreiben **nicht** direkt ins Journal oder die Hauptdatenbank.
3. Alle externen Wirkungen (Mail senden, Upload, Festschreibung) laufen über Approval + Gate.
4. Adapter sprechen nur den **Adaptervertrag** / Port, nie Domäneninternals.

---

## 4. Datenfluss (Kernworkflow)

```text
Bank/PDF/GT Import
    → ImportBatch + SourceTransaction / Invoice
    → EconomicTransaction (Dedup / Merge prüfen)
    → Matching Candidates (+ optional AI suggest)
    → Document link / MissingDocumentCase
    → Extraction (OCR) → BookingDraft
    → User Approval → JournalEntry (posted)
    → Reports / TaxReturnDraft / ExportBatch
```

Nebenläufig:
- Reminder-Jobs lesen MissingDocumentCase.
- Portal/Mail-Connectoren erzeugen Document-Imports (read/download only nach Capability).
- Backup-Job sichert DB + BlobStore + Config/Regelversionen.

---

## 5. Persistenzmodell

### 5.1 SQLite

- Eine Primärdatenbank pro BusinessProfile/Datenverzeichnis.
- Schema-Versionierung über Migrations (aufwärts; Rollback-Strategie dokumentiert).
- FTS für Dokument-/Transaktionstext; semantischer Index optional und rebuildbar.
- At-rest Encryption Ziel: SQLCipher o.ä. (Pin in M01 ADR, D-20).

### 5.2 Content-Addressed Object Store

```text
<data_root>/objects/ab/cd/<sha256>
<data_root>/derived/<sha256>/ocr.json | preview.webp | ...
```

- Object-Key = Content-Hash.
- Derived artifacts nie als „Original“.
- Import speichert `Document` Metadaten in SQLite mit Hash-Referenz.

### 5.3 Geld & Zeit

- Geldbeträge: Minor-unit Integers + Currency-Code; FX mit höherer Präzision, Quelle, Datum (D-22).
- Timestamps UTC; fachliche Datumsfelder Europe/Berlin (D-21).

---

## 6. Permission- und Credential-Modell

```text
Caller (UI|CLI|API|MCP|Plugin)
   → Permission Gate (action, resource, scope, payload_hash)
   → Approval required? → ActionApproval record
   → Service execution
   → AuditEvent
```

Credentials:
- Gespeichert nur als `CredentialReference` + Keychain-Eintrag.
- Plugins erhalten kurze, scoped Tokens/Handles vom Broker — keine Klartext-Secrets im Plugin-State dump.

---

## 7. Plugin-Host (Kurz)

- Separater Prozess; IPC über versionierte JSON-Messages (siehe Plugin-Contract).
- Capability Negotiation bei Install/Start.
- Resource limits: CPU time, memory, network domain allowlist, FS allowlist.
- Crash des Hosts darf SQLite-Journal und Object Store nicht korrumpieren (WAL + fsync-Policy).

---

## 8. Lokale API-Oberflächen

| Surface | Bindung | Auth |
|---|---|---|
| REST `/v1/*` | loopback default | lokales Token + Origin/Host-Checks |
| MCP | lokale Session | dieselben Scopes wie REST |
| CLI | process-local | User-Session; keine Secrets in argv |

Netzwerk-Exposure nur nach expliziter Einstellung (nicht v1-Default).

---

## 9. macOS-Integrationen

| Bedarf | Ansatz |
|---|---|
| MoneyMoney | Begrenzter AppleScript/Export-Adapter (kein generischer Executor) |
| Apple Mail | M10 Capability-Probe; sonst IMAP-Provideradapter |
| Share / Watched folders | nach Bedarf ab M03+ |
| Keychain | Credential Broker |
| Notarization | M14 Release-Kette |

---

## 10. Repo-Layout (Ziel `Gitit360/Belegwerk`)

```text
apps/desktop/                 # Tauri 2 + React
crates/domain/
crates/application/
crates/infrastructure/
crates/plugin-host/
packages/design-system/
packages/contracts/
packages/plugin-sdk/
plugins/examples/
plugins/connectors/
tests/fixtures/synthetic/
docs/...
```

---

## 11. Bedrohungsmodell (Skizze für M00)

| Bedrohung | Mitigation |
|---|---|
| Verlorener/gestohlener Mac | DB+Objekte verschlüsselt; Keychain; FileVault empfohlen (Org) |
| Kompromittiertes Plugin | Isolation, Capabilities, kein Journal-Write, Signatur/Dev-Mode |
| Bösartiger Beleg / Prompt Injection | Untrusted input; Gate; keine Tool-Escalation aus Modelloutput |
| Browser-Automation Drift | Domain allowlist; Stop bei CAPTCHA/Layout/fremd |
| Backupverlust | RPO≤24h Messziel; Restore-Drill; Schlüsselstrategie dokumentieren |
| Fehlbedienung | Vorschau, Approvals, Undo nur für Entwürfe |

Vollständiges `SECURITY_MODEL.md` entsteht im Repo ab M01.

---

## 12. Offene Architektur-Pins (nicht blockierend für Spec-Abnahme)

1. Exakte SQLCipher- vs. Alternative-Library (M01 ADR).
2. Konkrete macOS-Sandbox-/Prozessisolationsmechanik für Plugin-Host (M02 Nachweis).
3. Ob CLI als separates Binary oder `belegwerk` Subcommand der App (M01).

Diese Punkte ändern nicht den gewählten Stack (D-10).

---

## 13. Abnahme dieses Vorschlags

Nutzer bestätigt in M00:
- Tauri 2 + React + TS + Rust + SQLite local-first.
- Plugin-first Ports.
- Kein Produktcode bis Freigabe.

---

*Ende Architekturvorschlag v1.1.*
