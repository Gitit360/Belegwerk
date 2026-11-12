# ADR-0001 – Application Stack

Status: **Accepted** (D-10)  
Date: 2026-11-12  
Deciders: Engineering default confirmed for M00/M01 start

## Context

Belegwerk needs a local-first macOS desktop app with a deterministic accounting core, React UI, and plugin host. Options considered: Electron+React, Tauri 2+React+TS+Rust, SwiftUI native.

## Decision

Use **Tauri 2 + React + TypeScript** for the shell/UI and **Rust** crates for domain/application/infrastructure. Local database: **SQLite**.

## Consequences

- Repo layout: `apps/desktop/`, `crates/*`, `packages/*`, `plugins/*`.
- Smaller binary and clearer OS permission boundaries than Electron.
- WebView limits must be documented; native macOS features go through Tauri commands / Rust.
- SwiftUI only via a future ADR if Tauri becomes a hard blocker.
- Full desktop window build is **macOS-primary**; Linux CI may compile Rust crates and TypeScript without a full Tauri GUI verify.

## Alternatives rejected

- Electron: larger attack surface / heavier bundle.
- SwiftUI-only: slower shared design-system reuse; deferred.
