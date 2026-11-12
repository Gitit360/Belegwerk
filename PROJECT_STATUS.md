# Belegwerk – Project Status

Stand: 12. November 2026 (Europe/Berlin)  
Repository: `Gitit360/Belegwerk`

## Pull requests

| PR | Branch | Focus |
|---|---|---|
| [#1](https://github.com/Gitit360/Belegwerk/pull/1) | `cursor/m00-m01-scaffold-0636` | M00 docs + scaffold (CI green) |
| [#2](https://github.com/Gitit360/Belegwerk/pull/2) | `cursor/vertical-slices-m02-m13-0636` | Gate/import/match/journal/CLI (+ M04/M06/HTTP) |
| [#4](https://github.com/Gitit360/Belegwerk/pull/4) | `cursor/modules-m04-m15-depth-0636` | M04–M15 depth + fixture nav pages (stacked on #2) |

---

## Modulstatus

| Modul | Status | Hinweis |
|---|---|---|
| M00 | done | Abnahme-Checkboxen offen |
| M01 | landed | Tokens + Nav-Shell |
| M02 | deepened | CommandBus + Gate |
| M03 | deepened | ImportPipeline |
| M04 | deepened | Fixture replay MoneyMoney (no AppleScript) |
| M05 | deepened | MatchingEngine + workbench |
| M06 | deepened | ReminderService + JobQueue |
| M07 | deepened | Labeled extraction + search index rebuild |
| M08 | deepened | AiRouter offline fallback (force_offline on CI) |
| M09 | deepened | GrandTotal JSON fixture import; DB writes forbidden |
| M10 | deepened | Mail search/draft + portal probe stubs |
| M11 | deepened | Balanced journal; blocked unverified tax |
| M12 | deepened | Report shells + drill-down stubs |
| M13 | deepened | OpenAPI + CLI + HTTP stub |
| M14 | deepened | Backup/restore roundtrip test (synthetic) |
| M15 | deepened | Extension hook registry (all disabled) |

## Policy

Keine Bankzahlungen · synthetische Fixtures only · keine erfundenen Steuerregeln · keine GoBD-Claims

## macOS-only gaps

Tauri GUI · MoneyMoney AppleScript · Keychain · Apple Mail
