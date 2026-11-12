# M00 Abnahme – Discovery und Spezifikation

Stand: 12. November 2026 (Europe/Berlin)

## Lieferumfang

- [x] Spec-Paket v1.1 ins Repo gespiegelt (`docs/product/*`, Architektur, Plugin, Gap, Traceability, Workplan)
- [x] `PROJECT_STATUS.md`, `OPEN_QUESTIONS.md`
- [x] `docs/product/REQUIREMENTS.md`, `SOURCES.md`
- [x] `docs/architecture/DECISIONS.md` + ADR-0001…0003 stubs
- [x] `docs/security/SECURITY_MODEL.md`
- [x] `docs/plugins/PLUGIN_SPEC.md`
- [x] `docs/operations/BACKUP_RESTORE.md`
- [x] Fixture-Policy (synthetic only)
- [x] Keine Fake-GoBD-/Steuerclaims

## Nutzerbestätigung (bitte abhaken)

- [ ] Begriffe: Belegwerk; Proxy = `DocumentPlaceholder`; Meta-Nodes relational
- [ ] Produktgrenzen inkl. EU-B2B out of v1; keine Bankzahlung
- [ ] Rechtepolitik (§4) und Plugin-Isolation
- [ ] Stack: Tauri 2 + React + TypeScript + Rust + SQLite
- [ ] Decision Register gelesen; Einwände dokumentiert oder akzeptiert
- [ ] Traceability deckt §0–§31 ab
- [ ] Offen nur bewusst deferred Items in `OPEN_QUESTIONS.md`
- [ ] Schriftliche Freigabe: M00 abgenommen / M01 darf fortgesetzt werden

## Freigabehinweis

Der Nutzer hat am 12.11.2026 ausdrücklich Entwicklung gestartet (M00 Docs + M01 Scaffold + Overnight-Weiterarbeit). Formale Checkbox-Abnahme kann morgens nachgezogen werden.

## Nicht Teil von M00

Produktivcode war in M00 verboten; M01-Scaffold ist ein **separates**, freigegebenes Arbeitspaket auf demselben Entwicklungsstrang.
