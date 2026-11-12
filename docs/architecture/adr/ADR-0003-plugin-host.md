# ADR-0003 – Plugin Host Isolation

Status: **Proposed** (spike questions for M02)  
Date: 2026-11-12  
Related: Plugin Contract v1.1, REQ-PL-001…003

## Context

Plugins (bank, invoice, document, OCR, AI, export, …) must extend Belegwerk without touching the journal core. Crash isolation and capability gating are mandatory.

## Direction (not final transport)

- Separate **plugin-host** process.
- Versioned JSON messages (`sdkVersion: "1"`).
- Capability negotiation at install/start; runtime enforcement in the application permission gate.
- Plugins never receive raw SQL / journal write handles.
- Unsigned plugins: Development Mode only with UI warning.

## Spike questions (M02)

1. IPC: stdin/stdout JSON Lines vs. Unix domain socket?
2. macOS sandbox / process isolation mechanics available to Tauri sidecar?
3. Resource limits: CPU, memory, network domain allowlist, FS allowlist — enforcement mechanism?
4. How does runtime capability revocation abort in-flight jobs?

## Interim M01 deliverable

- Manifest / message TypeScript (+ Rust mirror) types in `packages/plugin-sdk` and `packages/contracts`.
- Empty connector stubs (MoneyMoney, GrandTotal) behind the contract — **no live calls**.
- Host process stub crate `crates/plugin-host` that can handshake in-process for tests only.

## Decision deferred

Exact IPC transport and sandbox pin — do not block scaffold.
