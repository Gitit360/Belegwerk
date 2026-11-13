# Belegwerk – Fallmatrix v1

Stand: 13. November 2026  
Bezug: Product Description §15.2 · Register: `TAX_RULE_REGISTER_v2026.md`

Status: `supported` · `user_confirm` · `blocked`

| Fall | Status | Rule-IDs | Hinweis |
|---|---|---|---|
| Inländische Betriebsausgabe mit Beleg, Satz 19 % | supported | TAX-UST-RATE-19, TAX-EUR-11 | Journal-Draft |
| Inländische Einnahme Istversteuerung | supported | TAX-IST-20, TAX-UST-RATE-19 | Profil Ist |
| Ermäßigter Satz 7 % | user_confirm | TAX-UST-RATE-7 | Leistungsart bestätigen |
| Teilzahlung / Restbetrag Matching | supported | — | PaymentAllocation (M05) |
| Storno / Gutschrift | user_confirm | — | |
| Erstattung | user_confirm | — | |
| Private Anteile / gemischte Nutzung | supported | TAX-PRIVATE-SHARE | Privatquote 0–100 % Pflicht; Split Aufwand/Entnahme |
| Einlagen / Entnahmen (rein) | supported | TAX-PRIVATE-SHARE | Entnahmekonto bei Quote > 0 |
| Reverse Charge inbound (z. B. US-SaaS) | user_confirm | TAX-RC-13B | |
| Fremdwährung + Gebühren | user_confirm | — | |
| GWG ≤ 800 € Sofortabschreibung | user_confirm | TAX-GWG-800 | |
| Anlagen > GWG / AfA | blocked | TAX-PC-1Y optional | |
| Periodenwechsel ReportingSchedule | supported | TAX-VA-THRESH-2025, TAX-DFV | |
| EU-B2B-Verkauf | blocked | TAX-BLOCK-EU-B2B | D-06 |
| Vorsteuer aus OCR allein | blocked | TAX-BLOCK-VAT-FROM-OCR | |

Unsupported → `BlockedUnsupported`, nie stilles Mapping (REQ-B-004).
