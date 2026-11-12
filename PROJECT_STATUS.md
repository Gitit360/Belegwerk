# Belegwerk – Project Status

Stand: 12. November 2026 (Europe/Berlin)  
Repository: `Gitit360/Belegwerk`  
Default branch: `main`

---

## Modulstatus

| Modul | Name | Status | Hinweis |
|---|---|---|---|
| M00 | Discovery und Spezifikation | **in review** | Docs im Repo gespiegelt; Abnahme via `docs/acceptance/M00_ACCEPTANCE.md` |
| M01 | Repository, Architektur und Designsystem | **in progress** | Minimaler Tauri-/Rust-/React-Skeleton (kein Fach-UI) |
| M02–M15 | siehe Product Description §26 | not started | Gates unverändert |

---

## Aktuelle Freigabe

Nutzer hat Entwicklung gestartet (Repo URL + Auftrag M00 Docs + M01 Scaffold).  
Keine GoBD-/Steuerkonformitäts-Claims. Keine produktiven Belege oder Credentials im Repo.

---

## Nächste Schritte

1. Nutzer: M00-Abnahmechecklist in `docs/acceptance/M00_ACCEPTANCE.md` bestätigen.
2. Engineering: M01 vertiefen (CI green auf macOS, Design Tokens, Migrationsgerüst) — ohne Buchhaltungsfachlogik.
3. Bewusst deferred Items: `OPEN_QUESTIONS.md`.

---

## Policy-Erinnerung

- Deutsche Produktdoku; englische Code-IDs.
- Nur synthetische/anonymisierte Fixtures (`tests/fixtures/synthetic/`).
- Keine Bankzahlungsfunktionen.
- Keine erfundenen Steuerregeln.
