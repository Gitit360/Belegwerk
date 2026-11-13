# macOS Smoke Checklist

Stand: 13. November 2026

Voraussetzung: Apple Silicon native (`pnpm check:silicon` → OK; `rustc -vV` host `aarch64-apple-darwin`).

1. `pnpm install`
2. `pnpm tauri:dev` — Fenster öffnet, Nav sichtbar; Activity Monitor: Belegwerk = Apple

3. Einstellungen → TaxProfile speichern
4. Konten → „Bank-Fixtures importieren“ → Bewegungen sichtbar
5. Eingang → Textdatei droppen → Batch + Dokument
6. Abgleich → „Vorschläge neu berechnen“ → Bestätigen (Privatanteil optional)
7. Buchhaltung → Draft aus Match sichtbar; Approve nur mit Freigabe
8. Berichte / Archiv → Daten aus SQLite
9. Einstellungen → macOS Capability-Probes lesen
10. Live-Test (MoneyMoney + Mail müssen laufen):

```bash
# laufende tauri:dev-Session stoppen, dann:
BELEGWERK_LIVE_CONNECTORS=1 pnpm tauri:dev
```

11. Einstellungen → Probes: `moneymoney` / `apple_mail` = verfügbar, Live = an  
12. Konten → **MoneyMoney Live (30 Tage)** → Umsätze erscheinen  
13. Einstellungen → **Keychain-Slot prüfen** + **Mail Live „Rechnung“**  
14. Abgleich → Vorschläge neu berechnen (Live-Umsätze + Belege)

Ohne Flag bleibt Fixture-Pfad Default. Live ist read-only (kein `create bank transfer`).
