# Belegwerk – Steuerregel-Register v2026

Stand Recherche: 13. November 2026 (Europe/Berlin)  
Status: Engineering-Register mit Quellen.

Encode-Status: `default_after_confirm` · `suggest_only` · `blocked`

---

## High confidence → versionierte Defaults (nach Nutzerbestätigung)

| ID | Aussage | Quelle | Gültigkeit | Confidence | Encode |
|---|---|---|---|---|---|
| TAX-UST-RATE-19 | Regelsteuersatz 19 % der Bemessungsgrundlage | [§12 Abs. 1 UStG](https://www.gesetze-im-internet.de/ustg_1980/__12.html) | aktuell | high | `default_after_confirm` → `standard_19` |
| TAX-UST-RATE-7 | Ermäßigter Satz 7 % (§12 Abs. 2 / Anlage 2) | §12 Abs. 2 UStG | aktuell | high | Katalog; nur bei bestätigter Leistungsart |
| TAX-IST-20 | Istversteuerung nach vereinnahmten Entgelten; FA-Gestattung; freie Berufe §20 Nr. 3 | [§20 UStG](https://www.gesetze-im-internet.de/ustg_1980/__20.html) | aktuell | high | Profil `vat_method=ist` |
| TAX-EUR-11 | EÜR: Zufluss Einnahmen / Abfluss Ausgaben | [§11 EStG](https://www.gesetze-im-internet.de/estg/__11.html) | aktuell | high | Timing EÜR |
| TAX-VA-THRESH-2025 | Monats-VA wenn Vorjahressteuer > 9.000 €; sonst Quartal; Befreiung möglich ≤ 2.000 € | §18 Abs. 2 UStG; BEG IV / BMF | ab 01.01.2025 | high | ReportingSchedule-Schwellen |
| TAX-DFV | Dauerfrist +1 Monat; Monatszahler 1/11 Sondervorauszahlung | §46 ff. UStDV | aktuell | high | Default aus (D-05) |
| TAX-GWG-800 | Sofortabschreibung GWG ≤ 800 € netto; Sammelposten 250,01–1.000 | §6 Abs. 2/2a EStG; EStH 2025 | 2025/2026 | high | Jahresbezogene Grenze |
| TAX-PRIVATE-SHARE | Gemischte Nutzung / Privatanteil: betrieblicher Anteil = 100 % − Privatquote; Vorsteuer anteilig; Privatanteil als Entnahme | Praxis EÜR/USt; Quote vom Nutzer gesetzt | v1 | high | `default_after_confirm` — Quote 0–100 % Pflichtfeld |

## Medium → vorschlagen, nie auto-buchen

| ID | Aussage | Quelle | Encode |
|---|---|---|---|
| TAX-KU-2025 | Kleinunternehmer Vorjahr ≤ 25.000 / lfd. ≤ 100.000 | [§19 UStG](https://www.gesetze-im-internet.de/ustg_1980/__19.html) | Setup-Wahl |
| TAX-RC-13B | Sonstige Leistung ausländischer Unternehmer → Empfänger oft Steuerschuldner | [§13b UStG](https://www.gesetze-im-internet.de/ustg_1980/__13b.html) | Falltyp + Bestätigung |
| TAX-PC-1Y | Computer/Software ND 1 Jahr (BMF 22.02.2022) | BMF | Nutzerwahl |
| TAX-NEW-MONTHLY | Neugründer oft monatliche VA | §18 UStG | Profil-Feld |

## Blocked bis Fachfreigabe

| ID | Grund |
|---|---|
| TAX-BLOCK-EU-B2B | EU-B2B-Verkäufe außerhalb v1 (D-06) |
| TAX-BLOCK-REDUCED-UNCONFIRMED | Ermäßigter Satz ohne bestätigte Leistungsart |
| TAX-BLOCK-VAT-FROM-OCR | Vorsteuer nur wegen Zahlung/OCR |
| TAX-BLOCK-GASTRO-2026 | Gastronomie-Satzwechsel ohne Perioden-Mapping |
| TAX-BLOCK-ERIC | ERiC-Direktübermittlung |
| TAX-BLOCK-ELSTER-FIELDS | Produktive ELSTER-Feldwerte ohne Formularversions-Check |
| TAX-BLOCK-SKR03-KEYS | SKR03-Steuerschlüssel ohne Lizenz-/Versionsprüfung |

---

*Maschinenlesbar: `crates/domain/rules/tax_profile_2026.json`.*
