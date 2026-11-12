# Belegwerk – Open Questions

Stand: 12. November 2026 (Europe/Berlin)  
Regel: Nur **echte macOS- oder externe Blocker**. Entschiedene Engineering-Defaults stehen in `docs/architecture/DECISIONS.md` und gehören nicht hierher.

---

| Thema | Warum blockiert (extern / macOS) | Wiederaufnahme |
|---|---|---|
| Tauri-Desktop-GUI auf echtem macOS | CI ist Linux; `tauri dev`/`tauri build` nicht in dieser Cloud-Umgebung verifizierbar | Vor erstem lokalen Smoke auf Mac |
| MoneyMoney Live-AppleScript (M04) | Erfordert macOS + installiertes MoneyMoney; Linux nur Fixture-Replay | M04 Live-Probe auf Nutzer-Mac |
| Keychain / Credential-Slot | macOS Keychain APIs; keine Secrets in Repo/CI | Vor Live-Connector-Auth |
| Apple Mail Search/Draft (M10) | macOS Mail + Automation-Rechte; Stub reicht für Linux CI | M10 Live-Probe |
| GrandTotal 10 Exportfelder (G-27) | Braucht laufende GT-10-Installation und Probe-Export | M09 Live-Probe |
| ERiC-Direktübermittlung (G-26) | Externe Registrierung + Plattformcheck, kein In-Repo-Mock | M15 |
| Finale OSS-/Kommerzlizenz (D-09) | Nutzer-/Rechtsentscheid, kein Code-Default | Vor öffentlicher SDK-Freigabe |
| Marken-/Namensrechte „Belegwerk“ (G-25) | Rechtliche Prüfung | Vor öffentlichem Marketing |
| Produktiv-Steuerwerte (Sätze, GWG je Jahr) | Quellenpflicht; keine erfundenen Rechtsgewissheiten | Vor produktivem M11 |
| Bank X konkrete Schnittstelle (G-28) | Phase-2 Extern; MoneyMoney zuerst | M15 |

---

*Keine Engineering-Nits (SQLCipher-Pin, CLI-Layout, Plugin-IPC-Spike, Node-Graph-UX) — die sind entschieden oder bewusst deferred in den ADRs.*
