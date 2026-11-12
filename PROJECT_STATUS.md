# Belegwerk – Project Status

Stand: 12. November 2026 (Europe/Berlin)  
Repository: `Gitit360/Belegwerk`  
Branch: `cursor/m00-m01-scaffold-0636` (overnight full-surface push)

---

## Modulstatus

| Modul | Name | Status | Hinweis |
|---|---|---|---|
| M00 | Discovery / Spec | **done (docs in repo)** | Abnahme-Checkboxen in `docs/acceptance/M00_ACCEPTANCE.md` |
| M01 | Repo / Designsystem / Skeleton | **landed** | Tokens, Nav-Shell, migrations, CI |
| M02 | Plugin-Host / Rechte | **stub landed** | In-process host + Permission Gate; IPC pin open |
| M03 | Archiv / Import | **stub landed** | FsBlobStore + DocumentMeta + ImportBatch |
| M04 | MoneyMoney | **stub landed** | Synthetic fixtures; live AppleScript = macOS gap |
| M05 | Matching / Splits / UI | **types + nav** | StatusDimensions; no full workbench UI yet |
| M06 | Proxy / Reminder | **stub landed** | DocumentPlaceholder + MissingDocumentCase |
| M07 | OCR / Search | **stub landed** | ExtractionResult without inventing fields |
| M08 | KI-Adapter | **interface landed** | NullAiProvider offline; Ollama port unwired |
| M09 | GrandTotal | **stub landed** | File adapter stub; no proprietary DB |
| M10 | Mail / Portale | **stub only** | Capabilities in plugin spec; no live mail |
| M11 | Buchhaltungskern | **structures landed** | SKR03 seed accounts; posting blocked without reviewed rules |
| M12 | Steuerentwürfe / Berichte | **shells landed** | TaxReturnDraft shells; no form field invention |
| M13 | Export / API / MCP | **stubs landed** | OpenAPI stub + export package stub + MCP note |
| M14 | Härtung | **partial** | Backup doc + migration tests; restore drill pending macOS |
| M15 | Extensions | **hooks only** | Bank X / Steuerboard / ERiC / EU-B2B blocked or deferred |

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
