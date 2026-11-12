# Belegwerk – Project Status

Stand: 12. November 2026 (Europe/Berlin)  
Repository: `Gitit360/Belegwerk`

## Pull requests

| PR | Branch | Focus | CI |
|---|---|---|---|
| [#1](https://github.com/Gitit360/Belegwerk/pull/1) | `cursor/m00-m01-scaffold-0636` | M00 docs + scaffold foundation | **green** on `0ff457d` |
| [#2](https://github.com/Gitit360/Belegwerk/pull/2) | `cursor/vertical-slices-m02-m13-0636` | Working vertical slices (stacked on #1) | in progress / iterate |

---

## Modulstatus

| Modul | Name | Status | Hinweis |
|---|---|---|---|
| M00 | Discovery / Spec | **done** | Abnahme-Checkboxen offen |
| M01 | Repo / Designsystem / Skeleton | **landed (#1)** | Tokens, Nav-Shell, CI |
| M02 | Plugin-Host / Rechte | **deepened** | CommandBus + Gate + audit |
| M03 | Archiv / Import | **deepened** | ImportPipeline end-to-end |
| M04 | MoneyMoney | **deepened (Linux)** | BankImportService + synthetic upsert; AppleScript macOS gap |
| M05 | Matching / Splits / UI | **deepened** | Engine + three-pane workbench |
| M06 | Proxy / Reminder | **deepened** | ReminderService cases/placeholders/escalation |
| M07 | OCR / Search | stub | Extraction without inventing fields |
| M08 | KI-Adapter | interface | NullAiProvider offline |
| M09 | GrandTotal | stub | File adapter stub |
| M10 | Mail / Portale | stub | Slots only |
| M11 | Buchhaltungskern | **deepened** | Balanced journal; blocked unverified tax |
| M12 | Steuerentwürfe / Berichte | shells | No form field invention |
| M13 | Export / API / MCP | **deepened** | OpenAPI + CLI + loopback HTTP stub `/v1/health` + 403 payments |
| M14 | Härtung | partial | Backup helper |
| M15 | Extensions | hooks | Bank X / Steuerboard / ERiC |

---

## Policy

- Keine Bankzahlungen · keine Credentials · nur synthetische Fixtures  
- Keine erfundenen Steuerregeln · keine Fake-GoBD-Claims  
- UI Deutsch / Code Englisch  

## macOS-only gaps

- Tauri window / notarization  
- MoneyMoney AppleScript  
- Keychain credential broker  
- Apple Mail connector probe  
