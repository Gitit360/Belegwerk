# Belegwerk

Lokale Mac-Buchhaltung für freiberufliche EÜR mit Belegabgleich.

**Stack:** Tauri 2 · React · TypeScript · Rust · SQLite  
**Sprache:** UI/Docs Deutsch · Code-IDs Englisch  

Dieses Repository startet mit Spezifikation (M00) und einem lauffähigen Gerüst plus Modul-Stubs (M01–M15 Oberflächen). Es enthält **keine** erfundenen Steuerregeln, **keine** Bankzahlungen und **keine** produktiven Belege.

## Voraussetzungen (macOS)

- **Apple Silicon (arm64)** — natives Build-Ziel; kein Rosetta-/x86_64-Rust
- macOS 12+
- Xcode CLT (arm64)
- Node 20+ und [pnpm](https://pnpm.io) 10+
- Rust stable via `rustup` mit Host `aarch64-apple-darwin` (Tauri 2)
- Systemabhängigkeiten für [Tauri 2](https://v2.tauri.app/start/prerequisites/)

```bash
# Einmalig, falls rustc noch x86_64/Rosetta ist:
rustup set default-host aarch64-apple-darwin
rustup toolchain install stable -c rustfmt -c clippy
# Prüfen: rustc -vV  →  host: aarch64-apple-darwin

pnpm install
pnpm check:silicon       # arm64 + natives rustc
pnpm check:rust          # Silicon-Check + fmt + clippy + tests
pnpm typecheck           # TypeScript packages / app
pnpm --filter @belegwerk/desktop dev   # Vite UI only
pnpm tauri:dev           # Full desktop window (macOS, aarch64)
```

## Linux / Cloud-CI Hinweis

Auf Linux-VMs ohne Tauri-Systembibliotheken (WebKitGTK etc.) kann `tauri build`/`tauri dev` fehlschlagen.  
**Erwartet grün:** `cargo test --workspace` (Default-Mitglieder: Domain/Application/Infrastructure/Plugin-Host) und `pnpm typecheck` / Vite build.  
MoneyMoney-AppleScript und echte macOS-Keychain-Pfade sind **macOS-only**.

## Repo-Layout

```text
apps/desktop/           Tauri 2 + React Shell
crates/domain/          Deterministische Domänentypen
crates/application/     Permission Gate, Approvals, Local API stubs
crates/infrastructure/  SQLite migrations, blob store
crates/plugin-host/     Handshake / capability filter stub
packages/design-system/ Tokens + Nav-IDs
packages/contracts/     Wire types
packages/plugin-sdk/    sdkVersion "1" types
plugins/…               Example + connector stubs
docs/                   Product & architecture
tests/fixtures/synthetic/   Nur synthetische Daten
```

## Sicherheit (Kurz)

- Keine `bank.payment.create`-Funktion (Gate verweigert dauerhaft)
- Secrets nie im Repo; nur Credential-Referenzen später via Keychain
- Ungeprüfte Steuerfälle → `BlockedUnsupported`
- Keine GoBD-/„rechtssicher“-Claims ohne Fachprüfung

## Dokumentation

Siehe [`docs/README.md`](./docs/README.md), [`PROJECT_STATUS.md`](./PROJECT_STATUS.md), [`OPEN_QUESTIONS.md`](./OPEN_QUESTIONS.md).
