# Belegwerk – Project Status

Stand: 13. November 2026 (Europe/Berlin)  
Repository: `Gitit360/Belegwerk`

## Pull requests (nicht mergen ohne Nutzer)

| PR | Branch | Focus | CI (letzter Stand) |
|---|---|---|---|
| [#1](https://github.com/Gitit360/Belegwerk/pull/1) | `cursor/m00-m01-scaffold-0636` | M00 docs + Scaffold | green |
| [#2](https://github.com/Gitit360/Belegwerk/pull/2) | `cursor/vertical-slices-m02-m13-0636` | Gate/import/match/journal/CLI | green |
| [#4](https://github.com/Gitit360/Belegwerk/pull/4) | `cursor/modules-m04-m15-depth-0636` | M04–M15 Depth + Fixture-UI + Hardening | green @ `74bf9eb` |
| [#5](https://github.com/Gitit360/Belegwerk/pull/5) | `cursor/core-workflows-abgleich-import-0636` | Core workflows (Abgleich/Import/Reminder/Journal/Settings) | in progress |

Merge-Reihenfolge wenn freigegeben: **#1 → #2 → #4 → #5** (Stack).

---

## Modulstatus

| Modul | Status | Hinweis |
|---|---|---|
| M00 | done | Abnahme-Checkboxen offen |
| M01 | landed | Tokens + Nav-Shell; DE empty/loading/error |
| M02 | hardened | CommandBus + Gate; Bank-Pay Deny + Audit Tests |
| M03 | workflow | ImportPipeline + Eingang Drop-Zone + Batch-Liste |
| M04 | deepened | Fixture replay MoneyMoney (no AppleScript) |
| M05 | workflow | Confirm/Reject, PaymentAllocation, Remainders, Batch-Vorschau |
| M06 | workflow | ReminderService escalate/complete in UI |
| M07 | deepened | Labeled extraction + search index rebuild |
| M08 | deepened | AiRouter offline fallback (`force_offline` on CI) |
| M09 | deepened | GrandTotal JSON fixture import; DB writes forbidden |
| M10 | deepened | Mail search/draft + portal probe stubs |
| M11 | workflow | Journal drafts UI; approve via Gate only |
| M12 | deepened | Report shells + drill-down stubs |
| M13 | deepened | OpenAPI + CLI + HTTP stub + Tauri commands |
| M14 | hardened | Backup/restore: schema + blob bytes integrity |
| M15 | deepened | Extension hook registry (all disabled) |
| Settings | workflow | TaxProfile (Ist/monatlich, Dauerfrist aus) in SQLite |

## Core workflows (#5+)

- Matching E2E + batch toolbar + rebuild + Privatanteil (TAX-PRIVATE-SHARE)
- Eingang drop-zone → ImportPipeline → match rebuild
- Konten: Bank-Fixture-Replay → economic_transactions
- Fehlende Belege escalate/complete
- Buchhaltung drafts aus Match + blocked EU-B2B
- Berichte/Archiv aus SQLite
- Einstellungen TaxProfile + macOS Capability-Probes
- Regelregister `docs/accounting/` + `crates/domain/rules/tax_profile_2026.json`
- Tauri invoke + browser fallbacks

## Policy

Keine Bankzahlungen · synthetische Fixtures only · keine erfundenen Steuerregeln · keine GoBD-Claims · **kein Merge ohne Nutzer**

## Fake vs. real (Kurz)

| Real / getestet in CI/Linux | Fake / Stub / macOS-only |
|---|---|
| Gate, Import, Match allocations, Journal store, Settings, Reminder complete, Vite UI | Tauri native window (commands wired), MoneyMoney AppleScript, Keychain, Apple Mail, GrandTotal live, ERiC |

## Toolchain (Apple Silicon)

- Host-Ziel: **aarch64-apple-darwin** (kein Rosetta/x86_64-Rust)
- Check: `pnpm check:silicon` · `scripts/ensure-apple-silicon.sh`
- `rust-toolchain.toml` → `stable` (arm64); Workspace + Tauri nativ

## Live-Connector (macOS)

- Flag: `BELEGWERK_LIVE_CONNECTORS=1`
- MoneyMoney: `export transactions … as "plist"` (read-only) → Konten → **MoneyMoney Live**
- Mail: Inbox-Suche (read-only) · Keychain-Slot-Marker
- Default ohne Flag: Fixture-Replay

## macOS-only gaps

Tauri GUI smoke · MoneyMoney AppleScript · Keychain · Apple Mail — Details in `OPEN_QUESTIONS.md`
