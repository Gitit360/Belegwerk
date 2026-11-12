# Belegwerk – Security Model (M00 Skizze)

Stand: 12. November 2026 (Europe/Berlin)  
Bezug: Architektur §11, Product Description §4 / §23, REQ-S-*

**Disclaimer:** Dies ist ein Engineering-Bedrohungsmodell, keine Zertifizierung und kein GoBD-/Rechtsnachweis.

---

## 1. Trust Boundaries

| Zone | Vertrauen | Beispiele |
|---|---|---|
| Domain / Application (Rust) | Hoch | Journal, Permission Gate, Approvals |
| Desktop UI (WebView) | Mittel | Darf nur typed commands; keine Steuerlogik |
| Plugins / Adapters | Niedrig | Isolierter Prozess; Capability-bound |
| Untrusted input | Null | Beleg-OCR, Web/Portal-Inhalte, Modelloutput |

## 2. Permission Gate

```text
Caller (UI|CLI|API|MCP|Plugin)
  → Permission Gate (action, resource, scope, payload_hash)
  → Approval required? → ActionApproval
  → Service execution
  → AuditEvent
```

UI, plugins, CLI and MCP **cannot bypass** the gate. Approvals bind action, target, payload hash, user, time, validity.

### Explicitly forbidden

- `bank.payment.create` (capability does not exist; must never be added without product decision)
- Silent mail send / portal delete / contract change
- Model-driven shell / AppleScript / unrestricted browser
- Secrets in DB, logs, prompts, argv

## 3. Credentials

- Stored as `CredentialReference` + macOS Keychain via Credential Broker.
- Plugins receive short-lived scoped handles — never long-lived plaintext dumps.
- No real credentials in this repository.

## 4. Data at rest

- SQLite encryption target (SQLCipher o.ä.) — library pin open (ADR-0002).
- Content-addressed originals; derived artifacts separate.
- FileVault recommended organizationally.

## 5. Threat sketch

| Threat | Mitigation |
|---|---|
| Lost/stolen Mac | Encrypted DB+objects; Keychain; FileVault |
| Compromised plugin | Isolation, capabilities, no journal write, signature/dev-mode |
| Malicious document / prompt injection | Untrusted input; gate; no tool escalation from model output |
| Browser-automation drift | Domain allowlist; stop on CAPTCHA/layout change |
| Backup loss | RPO≤24h / RTO≤4h goals; restore drills |
| Misuse | Preview, approvals, undo only for drafts |

## 6. Privacy

- No default telemetry (REQ-S-009).
- OpenRouter / cloud AI opt-in only with allowlist (M08).

## 7. Compliance language

GoBD-oriented process goals (original protection, history, posting locks, access control, recoverability) are **design intents**. Do **not** claim „GoBD-zertifiziert“ or tax-law compliance without independent professional review.
