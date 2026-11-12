# Database Migrations

- Engine: SQLite via `rusqlite` (bundled)
- Source of truth: `crates/infrastructure/src/db.rs` (`MIGRATIONS` array)
- Table `schema_migrations(version, applied_at)` tracks applied versions
- Additive only in v1; rollback strategy = restore from backup (see `BACKUP_RESTORE.md`)
- Encryption library pin (SQLCipher) still open — ADR-0002

## Current versions

| Version | Contents |
|---|---|
| 1 | batches, documents, source/economic transactions, matches, approvals, audit, jobs |
| 2 | chart_accounts, tax_rule_packs, journal shells, tax_return_drafts, missing docs, placeholders |
