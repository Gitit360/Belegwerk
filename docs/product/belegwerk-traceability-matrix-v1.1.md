# Belegwerk – Traceability-Matrix v1.1

Stand: 12. November 2026 (Europe/Berlin)  
Quelle: Product Description v1.1  
Regel: Jede Requirement-ID hat Modul(e), Abschnitt(e) und Abnahmetyp. Status hier = Spezifikationsstatus (nicht Code).

Legende Status: `Spec` = spezifiziert · `Gate` = Freigabe nötig vor Code · `Deferred` = bewusst später

---

## REQ – Produkt & Prozess

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-P-001 | Modulare Arbeit; kein Big-Bang | M00–M15 | §0, §26 | Prozess | Spec |
| REQ-P-002 | Vor Implementierung Modul-Anforderungen lesen | alle | §0 | Prozess | Spec |
| REQ-P-003 | Arbeitspaket-Beschreibung vor Änderungen | alle | §0, §28 | Prozess | Spec |
| REQ-P-004 | Tests ausführen und echte Ergebnisse protokollieren | alle | §0, §25, §28 | QA | Spec |
| REQ-P-005 | Meilenstein-Abnahme vor nächstem Modul | alle | §0, §26 | Gate | Spec |
| REQ-P-006 | Keine vermuteten Steuerregeln; Unklar → blockiert | M11+ | §0, §15 | Fachlich | Spec |
| REQ-P-007 | Nur synthetische/anonymisierte Dev-Daten | alle | §0 | Security | Spec |
| REQ-P-008 | Externe APIs vor Implementierung verifizieren | Adapter | §0, §30 | Gate | Spec |
| REQ-P-009 | Kein Produktcode vor M00-Freigabe | M00 | §0.1 | Gate | Gate |
| REQ-P-010 | Produktname Belegwerk; Docs DE / Code EN | M00–M01 | §1, D-01/D-14 | Spec | Spec |

---

## REQ – Vision & Grenzen

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-V-001 | Kernworkflow Import→Match→Buchung→Auswertung | M03–M12 | §1 | Funktional | Spec |
| REQ-V-002 | KI assistiert, entscheidet nicht steuerlich | M08, M11 | §1, §14 | Fachlich | Spec |
| REQ-B-001 | Zielumfang Archiv/Match/Buchhaltung/API/Export | M03–M13 | §3 | Scope | Spec |
| REQ-B-002 | Out-of-scope Lohn/Warenwirtschaft/ESt-gesamt | — | §3 | Scope | Spec |
| REQ-B-003 | EU-B2B-Verkäufe außerhalb v1 | M11, M15 | §3, D-06 | Scope | Spec |
| REQ-B-004 | Nicht unterstützte Fälle sichtbar, nie still mappen | M05, M11 | §3, §5 | Fachlich | Spec |

---

## REQ – Sicherheit & Freigaben

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-S-001 | Permission Gate im Kern; UI/Plugin/CLI/MCP können nicht umgehen | M02 | §4, §21 | Security | Spec |
| REQ-S-002 | Freigaben an Aktion/Ziel/Payload-Hash/User/Zeit gebunden | M02 | §4 | Security | Spec |
| REQ-S-003 | Secrets nur Credential Broker / Keychain | M02 | §4, §23 | Security | Spec |
| REQ-S-004 | Keine Bankzahlungsfunktion (auch nicht per Plugin) | M02–M04 | §4, §25 | Security | Spec |
| REQ-S-005 | Mail senden nur mit Freigabe | M10 | §4, §12 | Security | Spec |
| REQ-S-006 | Modelle ohne Shell/AppleScript/Browser-Vollzugriff | M08 | §4, §14 | Security | Spec |
| REQ-S-007 | Untrusted Input (Beleg/Web) darf Rechte nicht erweitern | M03, M07, M10 | §23 | Security | Spec |
| REQ-S-008 | DB at-rest Encryption + Keychain | M01, M14 | §23, D-20 | Security | Spec |
| REQ-S-009 | Keine Default-Telemetrie | M01 | §23 | Privacy | Spec |
| REQ-S-010 | Signierung/Notarisierung vor Prod-Distribution | M14 | §23 | Ops | Spec |

---

## REQ – UX & Designsystem

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-U-001 | Navigation 12 Hauptbereiche | M01, M05 | §6 | UX | Spec |
| REQ-U-002 | Dreispaltige Primärarbeitsansicht | M05 | §6.1 | UX | Spec |
| REQ-U-003 | DnD, Multi-Select, Batch-Vorschau, Undo für Entwürfe | M05 | §6.2 | UX | Spec |
| REQ-U-004 | Design Tokens vor Fachseiten | M01 | §6.3 | UX | Spec |
| REQ-U-005 | A11y: Fokus, Screenreader, Reduced Motion, Kontrast | M01+ | §6.2 | A11y | Spec |
| REQ-U-006 | UI-Sprache Deutsch / de-DE | M01 | D-14 | UX | Spec |

---

## REQ – Archiv & Dokumente

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-D-001 | Import mit Herkunft, Hash, Batch | M03 | §7 | Funktional | Spec |
| REQ-D-002 | Originale nie durch OCR/Preview ersetzen | M03, M07 | §5, §7 | Fachlich | Spec |
| REQ-D-003 | Byte-identische Dubletten erkennen | M03 | §7 | Funktional | Spec |
| REQ-D-004 | Content-addressierter Originalspeicher | M03 | §7, D-11 | Architektur | Spec |
| REQ-D-005 | Sichere Dateiverarbeitung (Limits, ZIP-Bomben, Path Traversal) | M03 | §7 | Security | Spec |
| REQ-D-006 | Aufbewahrung je Dokumentklasse konfigurierbar | M03, M14 | §7 | Compliance | Spec |
| REQ-D-007 | DocumentPlaceholder ≠ Original / kein Vorsteuer allein | M06 | §11, D-16 | Fachlich | Spec |

---

## REQ – Bank & Adapter

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-BK-001 | MoneyMoney Read-only, idempotent | M04 | §8.1 | Integration | Spec |
| REQ-BK-002 | Adaptervertrag Mindestfelder | M02, M04 | §8.2 | Vertrag | Spec |
| REQ-BK-003 | SourceTransaction ≠ EconomicTransaction | M04, M05 | §8.2 | Domäne | Spec |
| REQ-BK-004 | PDF-Auszüge nur getestete Parserprofile | M04+ | §8.3 | Integration | Spec |
| REQ-BK-005 | Kreditkartenausgleich keine Doppelausgabe | M05, M11 | §8.4, §25 | Fachlich | Spec |
| REQ-BK-006 | Bank X Professional Phase-2 | M15 | §8.5, D-03 | Deferred | Deferred |
| REQ-BK-007 | Neue Konten/Adapter ohne Kernänderung | M02, M04 | §2, §8 | Architektur | Spec |

---

## REQ – GrandTotal

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-GT-001 | GrandTotal führend für Ausgangsrechnungen | M09 | §9 | Fachlich | Spec |
| REQ-GT-002 | Adapter GT10 dateibasiert; keine proprietäre DB | M09 | §9, D-02 | Integration | Spec |
| REQ-GT-003 | Versionsprobe bei Setup | M09 | §9 | Integration | Spec |
| REQ-GT-004 | Keine Doppeleinnahme Bank + GT-Zahlungsstatus | M09, M05 | §9, §25 | Fachlich | Spec |

---

## REQ – Matching & Status

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-M-001 | Viele-zu-viele Matching | M05 | §10 | Domäne | Spec |
| REQ-M-002 | Matching-Reihenfolge 1–7 | M05, M08 | §10 | Domäne | Spec |
| REQ-M-003 | PaymentAllocation / BookingSplit / UsageAllocation | M05, M11 | §10.1 | Domäne | Spec |
| REQ-M-004 | Fünf Statusdimensionen | M05–M13 | §10.2 | Domäne | Spec |
| REQ-M-005 | Erklärbare Match-Vorschläge inkl. Unsicherheit | M05, M08 | §10 | UX/KI | Spec |

---

## REQ – Reminder, Mail, Portale

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-R-001 | MissingDocumentCase inkl. Eskalation | M06 | §12 | Funktional | Spec |
| REQ-R-002 | Beschaffungspriorität lokal→Mail→API→Browser | M06, M10 | §12 | Prozess | Spec |
| REQ-R-003 | Portalconnectoren isoliert; CAPTCHA/Layout → Stop | M10 | §12.1 | Security | Spec |
| REQ-R-004 | Connector-Slot `mobile-de` ohne Fake-Brand | M10, M15 | §12.1, D-07 | Spec | Spec |
| REQ-R-005 | Apple Mail: keine stillen Mail-Mutationen | M10 | §12.2 | Security | Spec |

---

## REQ – OCR / KI

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-O-001 | Extraktionspipeline mit Feldprovenienz | M07 | §13 | Funktional | Spec |
| REQ-O-002 | Keine erfundenen fehlenden Felder | M07 | §13 | Fachlich | Spec |
| REQ-AI-001 | Providervertrag Capabilities | M08 | §14.1 | Vertrag | Spec |
| REQ-AI-002 | Ollama Standard; Kern offline nutzbar | M08 | §14.2 | Funktional | Spec |
| REQ-AI-003 | OpenRouter opt-in, Allowlist, kein stiller Fallback | M08 | §14.3 | Privacy | Spec |
| REQ-AI-004 | Codex Dev-only, keine Prod-Abhängigkeit | M08 | §14.4, D-12 | Architektur | Spec |
| REQ-AI-005 | Cache/Limits/Schema-Validierung | M08 | §14.5 | Ops | Spec |

---

## REQ – Buchhaltung & Steuer

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-A-001 | Journal Soll/Haben; EÜR eigene Abbildung | M11 | §15.1 | Fachlich | Spec |
| REQ-A-002 | Geld ohne binäre Floats; Minor Units | M11 | §15.1, D-22 | Domäne | Spec |
| REQ-A-003 | Regelregister versioniert mit Quellen | M11 | §15.2 | Fachlich | Spec |
| REQ-A-004 | Fallmatrix v1 inkl. RC-SaaS/FX; EU-B2B raus | M11 | §15.2, D-06 | Fachlich | Spec |
| REQ-A-005 | TaxProfileVersion + ReportingSchedule | M11, M12 | §15.3, D-04 | Domäne | Spec |
| REQ-A-006 | Dauerfrist default aus | M11, M12 | §15.3, D-05 | Domäne | Spec |
| REQ-A-007 | Historische Festschreibungen unverändert bei Regelupdate | M11 | §5, §15, §25 | Fachlich | Spec |
| REQ-A-008 | Anlagen mit Jahresbezug; keine Universalgrenzen | M11 | §15.4 | Fachlich | Spec |
| REQ-A-009 | Nutzer = primärer Fachprüfer; Parallelperiode vor Prod | M11, M14 | D-18, §25 | Gate | Spec |

---

## REQ – Berichte, ELSTER, Übergaben

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-T-001 | TaxReturnDraft + Feldmapping | M12 | §16.1 | Funktional | Spec |
| REQ-T-002 | Formularquellen vor M12 verifizieren | M12 | §16.1, §30 | Gate | Spec |
| REQ-T-003 | ERiC-Direktübermittlung als separates Modul | M15 | §16.2 | Deferred | Deferred |
| REQ-RPT-001 | Steuer- vs. Managementberichte getrennt | M12 | §17 | UX/Fachlich | Spec |
| REQ-RPT-002 | Drill-down bis Beleg; Datenstand sichtbar | M12 | §17 | Funktional | Spec |
| REQ-X-001 | Kompatibilitätsmatrix pro Format | M13 | §18.1 | Docs | Spec |
| REQ-X-002 | Vollständiges Exportpaket inkl. Manifest/Checksummen | M13 | §18.2 | Funktional | Spec |
| REQ-X-003 | Restore-Fähigkeit getestet | M14 | §18.2, §24, §25 | Ops | Spec |
| REQ-X-004 | Steuerboard Post-M15 | M15 | §18.3, D-13 | Deferred | Deferred |

---

## REQ – Architektur, Plugin, API

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-AR-001 | Modularer Monolith; Tauri2+React+TS+Rust+SQLite | M00, M01 | §19, D-10 | Architektur | Spec |
| REQ-AR-002 | Schichten UI→App→Domain→Ports→Adapters | M01 | §19 | Architektur | Spec |
| REQ-AR-003 | Keine Steuerlogik in UI/Plugin | M02, M11 | §19, §20 | Architektur | Spec |
| REQ-PL-001 | Versioniertes Plugin-Konzept ab Projektbeginn | M02 | §20 | Architektur | Spec |
| REQ-PL-002 | Isolierte Plugin-Laufzeit; Crashisolation | M02 | §20.3, §25 | Security | Spec |
| REQ-PL-003 | Beispielplugin ohne Kernänderung | M02 | §20.4 | Abnahme | Spec |
| REQ-API-001 | REST /v1/* lokal, auth, scopes | M02, M13 | §21 | API | Spec |
| REQ-API-002 | MCP gleiche Rechte wie App-Dienste | M13 | §21 | API | Spec |
| REQ-API-003 | CLI JSON + Exitcodes; keine Secrets in argv | M13 | §21 | API | Spec |

---

## REQ – Datenmodell, Jobs, Backup

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-DM-001 | Kernentitätenliste mit stabilen IDs | M01+ | §22 | Domäne | Spec |
| REQ-DM-002 | UTC Zeitpunkte; Berlin Kalenderdaten | M01 | §22, D-21 | Domäne | Spec |
| REQ-J-001 | Persistente Jobqueue; idempotent/abbrechbar | M01, M03+ | §24 | Ops | Spec |
| REQ-J-002 | Sichtbare Behandlung Schlaf/Netz/MoneyMoney-Offline | M04+ | §24 | UX/Ops | Spec |
| REQ-BU-001 | Backup: DB, Originale, Config, Plugin-/Regelversionen | M14 | §24 | Ops | Spec |
| REQ-BU-002 | RPO ≤ 24 h; RTO ≤ 4 h (Messziele) | M14 | §24, D-08 | Ops | Spec |

---

## REQ – Tests & Modulgates

| ID | Anforderung | Modul | Abschnitt | Abnahme | Status |
|---|---|---|---|---|---|
| REQ-QA-001 | Pflichtfälle aus §25 | je Modul | §25 | QA | Spec |
| REQ-QA-002 | Keine Prod-Freigabe Kern ohne Parallelperiode/Restore/Export | M14 | §25 | Gate | Spec |
| REQ-MOD-000 | M00 Spec-Paket + Nutzerfreigabe | M00 | §26 | Gate | Gate |
| REQ-MOD-001…014 | Module M01–M14 laut Plan | M01–M14 | §26 | Gate | Spec |
| REQ-MOD-015 | Erweiterungen Bank X, EU-B2B, Steuerboard, ERiC, Graph | M15 | §26 | Deferred | Deferred |

---

## Abdeckungsnotiz

Alle inhaltlichen Abschnitte §0–§31 der Product Description v1.1 sind über mindestens eine REQ-ID referenzierbar. Detailmapping Code↔REQ entsteht ab M01 in `REQUIREMENTS.md` im Repo `Gitit360/Belegwerk`.

---

*Ende Traceability-Matrix v1.1.*
