# Belegwerk – Open Questions

Stand: 12. November 2026 (Europe/Berlin)  
Regel: Nur **bewusst deferred** Items (Gap-Analyse §3). Entschiedene Defaults stehen im Decision Register / `docs/architecture/DECISIONS.md`.

---

| Thema | Grund | Wiederaufnahme |
|---|---|---|
| Finale Open-Source-/Kommerzlizenz (D-09) | Nutzerentscheid; SDK-Doku bleibt versioniert | Vor öffentlicher SDK-Freigabe |
| Marken-/Namensrechte „Belegwerk“ (G-25) | Rechtliche Prüfung, kein Engineering-Blocker | Vor öffentlichem Marketing/Distribution |
| Konkrete Backup-Medien/Orte (G-24) | Hängt von Hardware und Ops-Präferenz ab | Vor Produktivbetrieb / M14 |
| GrandTotal exakte Exportfelder (G-27) | Erfordert laufende GT-10-Installation und Probe | M09 |
| Bank X konkrete Schnittstelle (G-28 / D-03) | Phase-2; MoneyMoney zuerst | M15 |
| EU-B2B-Fallregeln (D-06) | Außerhalb v1-Matrix; Steuerrecht verifizieren | Nach v1-Stabilität / M15 |
| Visuelle Node-Graph-UI (D-17) | UX-Entscheidung Nutzer; Listen reichen | Nach Nutzerbestätigung |
| ERiC-Direktübermittlung (G-26) | Externe Registrierung + Plattformcheck | M15 |
| Steuerboard-Anbindung (D-13) | Nicht kritisch für Kernzuverlässigkeit | Post-M15 |
| Produktiv-Steuerwerte (Sätze, GWG-Grenzen je Jahr) | Keine erfundenen Rechtsgewissheiten | Vor M11 mit Quellenprüfung |
| SQLCipher vs. Alternative (D-20) | Engineering-Default gesetzt; Library-Pin | M01 ADR |
| Plugin-Host IPC Transport (stdio vs. UDS) | Spike nötig | M02 |
| Plugin-Signatur-PKI | Vor Prod-Plugin-Distribution | M14 |
| CLI als separates Binary vs. Subcommand | Layout-Detail | M01 |

---

*Keine Steuerregeln oder Konformitätsaussagen in dieser Liste — nur offene Engineering-/Produktentscheidungen.*
