# Belegwerk – Project Status

Stand: 12. November 2026 (Europe/Berlin)  
Repository: `Gitit360/Belegwerk`

## Pull requests

| PR | Branch | Focus |
|---|---|---|
| [#1](https://github.com/Gitit360/Belegwerk/pull/1) | `cursor/m00-m01-scaffold-0636` | M00 docs + scaffold foundation |
| [#2](https://github.com/Gitit360/Belegwerk/pull/2) | `cursor/vertical-slices-m02-m13-0636` | Working vertical slices M02/M03/M05/M11/M13 (stacked on #1) |

---

## Modulstatus

| Modul | Name | Status | Hinweis |
|---|---|---|---|
| M00 | Discovery / Spec | **done (docs in repo)** | Abnahme-Checkboxen offen |
| M01 | Repo / Designsystem / Skeleton | **landed (#1)** | Tokens, Nav-Shell, CI |
| M02 | Plugin-Host / Rechte | **deepened** | CommandBus + Gate on all commands; bank-pay denied + audited |
| M03 | Archiv / Import | **deepened** | ImportPipeline: hash → blob → ImportBatch/Document rows |
| M04 | MoneyMoney | stub | Synthetic fixtures; AppleScript = macOS gap |
| M05 | Matching / Splits / UI | **deepened** | Amount/reference engine + three-pane fixture workbench |
| M06 | Proxy / Reminder | stub | Types only |
| M07 | OCR / Search | stub | Extraction without inventing fields |
| M08 | KI-Adapter | interface | NullAiProvider offline |
| M09 | GrandTotal | stub | File adapter stub |
| M10 | Mail / Portale | stub | Slots only |
| M11 | Buchhaltungskern | **deepened** | Balanced journal drafts; unverified tax blocked |
| M12 | Steuerentwürfe / Berichte | shells | No form field invention |
| M13 | Export / API / MCP | **deepened** | OpenAPI routes + CLI `health`/`status` |
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
