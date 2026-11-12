# ADR-0002 – Persistence Model

Status: **Accepted** (D-11); encryption library pin **open** (D-20)  
Date: 2026-11-12

## Context

Structured bookkeeping data and immutable document originals must coexist without silent mutation of source files.

## Decision

1. **SQLite** (schema via numbered migrations) for structured entities, jobs, approvals, audit.
2. **Content-addressed filesystem** for originals:

```text
<data_root>/objects/ab/cd/<sha256>
<data_root>/derived/<sha256>/…
```

3. Object key = content hash (SHA-256). Derived artifacts (OCR, preview) never replace originals.
4. Money: minor-unit integers + currency (D-22). Timestamps UTC; calendar dates `Europe/Berlin` (D-21).
5. At-rest DB encryption target: SQLCipher or equivalent + Keychain (D-20). **Exact crate pin deferred** until macOS Keychain integration spike.

## Consequences

- Migrations live under `crates/infrastructure/migrations/`.
- Blob store API is a port; first implementation is local FS stub.
- Backup/restore must cover DB + objects + config/rule versions (`docs/operations/BACKUP_RESTORE.md`).
- No productive Belege in fixtures; synthetic only.

## Open pin

SQLCipher vs. alternative library — track in `OPEN_QUESTIONS.md`; does not block empty schema / migrations table.
