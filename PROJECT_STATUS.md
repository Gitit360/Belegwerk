# Belegwerk – Project Status

Stand: 12. November 2026 (Europe/Berlin)  
Repository: `Gitit360/Belegwerk`

## Pull requests (nicht mergen ohne Nutzer)

| PR | Branch | Focus | CI (letzter Stand) |
|---|---|---|---|
| [#1](https://github.com/Gitit360/Belegwerk/pull/1) | `cursor/m00-m01-scaffold-0636` | M00 docs + Scaffold | green |
| [#2](https://github.com/Gitit360/Belegwerk/pull/2) | `cursor/vertical-slices-m02-m13-0636` | Gate/import/match/journal/CLI | green |
| [#4](https://github.com/Gitit360/Belegwerk/pull/4) | `cursor/modules-m04-m15-depth-0636` | M04–M15 Depth + Fixture-UI + Hardening | green @ `3de087a`; overnight follow-up pushed |

Merge-Reihenfolge wenn freigegeben: **#1 → #2 → #4** (Stack).

---

## Modulstatus

| Modul | Status | Hinweis |
|---|---|---|
| M00 | done | Abnahme-Checkboxen offen |
| M01 | landed | Tokens + Nav-Shell; DE empty/loading/error |
| M02 | hardened | CommandBus + Gate; Bank-Pay Deny + Audit Tests |
| M03 | hardened | ImportPipeline; Blob-Idempotenz + zwei Meta-Rows |
| M04 | deepened | Fixture replay MoneyMoney (no AppleScript) |
| M05 | hardened | Matching: Currency-/Tolerance-Edges |
| M06 | deepened | ReminderService + JobQueue |
| M07 | deepened | Labeled extraction + search index rebuild |
| M08 | deepened | AiRouter offline fallback (`force_offline` on CI) |
| M09 | deepened | GrandTotal JSON fixture import; DB writes forbidden |
| M10 | deepened | Mail search/draft + portal probe stubs |
| M11 | hardened | Journal balance + draft-pack gate |
| M12 | deepened | Report shells + drill-down stubs |
| M13 | deepened | OpenAPI + CLI + HTTP stub |
| M14 | hardened | Backup/restore: schema + blob bytes integrity |
| M15 | deepened | Extension hook registry (all disabled) |

## Overnight follow-up (diese Runde)

- Zusätzliche Unit-Tests: Permission Gate, Import-Idempotenz, Matching-Edges, Journal, Restore-Integrität
- Deutsche UI-Zustände (laden / leer / Fehler) auf Fixture-Seiten + Matching; Design-Tokens durchgängig
- `OPEN_QUESTIONS.md` auf echte macOS/externe Blocker reduziert
- Morning Briefing: `/cursor/stores/self/internal/morning-briefing.md`

## Policy

Keine Bankzahlungen · synthetische Fixtures only · keine erfundenen Steuerregeln · keine GoBD-Claims · **kein Merge ohne Nutzer**

## Fake vs. real (Kurz)

| Real / getestet in CI/Linux | Fake / Stub / macOS-only |
|---|---|
| Domain, Gate, Import, Match, Journal, SQLite, Blob, Backup/Restore, CLI doctor, Vite UI | Tauri native window, MoneyMoney AppleScript, Keychain, Apple Mail, GrandTotal live, ERiC, Steuerboard, Bank X |

## macOS-only gaps

Tauri GUI · MoneyMoney AppleScript · Keychain · Apple Mail — Details in `OPEN_QUESTIONS.md`
