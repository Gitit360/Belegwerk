use rusqlite::{Connection, OptionalExtension};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("migration failed: {0}")]
    Migration(String),
}

/// Ordered SQL migrations. Keep additive and documented.
pub const MIGRATIONS: &[&str] = &[
    // 1 — schema_migrations + core stubs
    r#"
    CREATE TABLE IF NOT EXISTS schema_migrations (
      version INTEGER PRIMARY KEY NOT NULL,
      applied_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS import_batches (
      id TEXT PRIMARY KEY NOT NULL,
      source_label TEXT NOT NULL,
      imported_at TEXT NOT NULL,
      note TEXT
    );

    CREATE TABLE IF NOT EXISTS documents (
      id TEXT PRIMARY KEY NOT NULL,
      content_hash TEXT NOT NULL,
      mime_type TEXT NOT NULL,
      original_filename TEXT,
      byte_size INTEGER NOT NULL,
      import_batch_id TEXT NOT NULL REFERENCES import_batches(id),
      imported_at TEXT NOT NULL,
      document_date TEXT,
      is_placeholder INTEGER NOT NULL DEFAULT 0
    );

    CREATE INDEX IF NOT EXISTS idx_documents_hash ON documents(content_hash);

    CREATE TABLE IF NOT EXISTS source_transactions (
      id TEXT PRIMARY KEY NOT NULL,
      provider_id TEXT NOT NULL,
      source_account_id TEXT NOT NULL,
      source_transaction_ref TEXT NOT NULL,
      booking_date TEXT NOT NULL,
      value_date TEXT,
      amount_minor INTEGER NOT NULL,
      currency TEXT NOT NULL,
      counterparty TEXT,
      purpose TEXT,
      status TEXT NOT NULL,
      imported_at TEXT NOT NULL,
      raw_ref TEXT,
      UNIQUE(provider_id, source_account_id, source_transaction_ref)
    );

    CREATE TABLE IF NOT EXISTS economic_transactions (
      id TEXT PRIMARY KEY NOT NULL,
      amount_minor INTEGER NOT NULL,
      currency TEXT NOT NULL,
      booking_date TEXT NOT NULL,
      description TEXT,
      status TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS economic_source_links (
      economic_transaction_id TEXT NOT NULL REFERENCES economic_transactions(id),
      source_transaction_id TEXT NOT NULL REFERENCES source_transactions(id),
      PRIMARY KEY (economic_transaction_id, source_transaction_id)
    );

    CREATE TABLE IF NOT EXISTS match_candidates (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      economic_transaction_id TEXT NOT NULL,
      document_id TEXT NOT NULL,
      confidence TEXT NOT NULL,
      reason TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS action_approvals (
      id TEXT PRIMARY KEY NOT NULL,
      action TEXT NOT NULL,
      resource_kind TEXT NOT NULL,
      resource_id TEXT,
      payload_hash TEXT NOT NULL,
      approved_by TEXT NOT NULL,
      approved_at TEXT NOT NULL,
      valid_until TEXT
    );

    CREATE TABLE IF NOT EXISTS audit_events (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      at TEXT NOT NULL,
      caller TEXT NOT NULL,
      action TEXT NOT NULL,
      decision TEXT NOT NULL,
      detail TEXT
    );

    CREATE TABLE IF NOT EXISTS jobs (
      id TEXT PRIMARY KEY NOT NULL,
      kind TEXT NOT NULL,
      status TEXT NOT NULL,
      payload_json TEXT NOT NULL,
      created_at TEXT NOT NULL,
      updated_at TEXT NOT NULL
    );
    "#,
    // 2 — accounting / tax draft shells (no invented rates)
    r#"
    CREATE TABLE IF NOT EXISTS chart_accounts (
      code TEXT PRIMARY KEY NOT NULL,
      name TEXT NOT NULL,
      kind TEXT NOT NULL,
      skr_variant TEXT NOT NULL DEFAULT 'SKR03'
    );

    CREATE TABLE IF NOT EXISTS tax_rule_packs (
      id TEXT PRIMARY KEY NOT NULL,
      version TEXT NOT NULL,
      status TEXT NOT NULL,
      source_note TEXT,
      UNIQUE(id, version)
    );

    CREATE TABLE IF NOT EXISTS journal_entries (
      id TEXT PRIMARY KEY NOT NULL,
      booked_at TEXT,
      status TEXT NOT NULL,
      description TEXT,
      rule_pack_id TEXT,
      rule_pack_version TEXT
    );

    CREATE TABLE IF NOT EXISTS journal_lines (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      journal_entry_id TEXT NOT NULL REFERENCES journal_entries(id),
      account_code TEXT NOT NULL,
      debit_minor INTEGER NOT NULL DEFAULT 0,
      credit_minor INTEGER NOT NULL DEFAULT 0,
      currency TEXT NOT NULL DEFAULT 'EUR'
    );

    CREATE TABLE IF NOT EXISTS tax_return_drafts (
      id TEXT PRIMARY KEY NOT NULL,
      tax_kind TEXT NOT NULL,
      period TEXT NOT NULL,
      status TEXT NOT NULL,
      form_version TEXT,
      notes TEXT
    );

    CREATE TABLE IF NOT EXISTS missing_document_cases (
      id TEXT PRIMARY KEY NOT NULL,
      economic_transaction_id TEXT,
      status TEXT NOT NULL,
      escalation_level INTEGER NOT NULL DEFAULT 0,
      note TEXT,
      created_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS document_placeholders (
      id TEXT PRIMARY KEY NOT NULL,
      case_id TEXT,
      label TEXT NOT NULL,
      created_at TEXT NOT NULL,
      CHECK (1)
    );
    "#,
    // 3 — core workflow persistence (match decisions, allocations, settings, journal flags)
    r#"
    CREATE TABLE IF NOT EXISTS match_decisions (
      id TEXT PRIMARY KEY NOT NULL,
      economic_transaction_id TEXT NOT NULL,
      document_id TEXT NOT NULL,
      decision TEXT NOT NULL,
      confidence TEXT NOT NULL,
      reason TEXT NOT NULL,
      decided_at TEXT NOT NULL,
      UNIQUE(economic_transaction_id, document_id)
    );

    CREATE TABLE IF NOT EXISTS payment_allocations (
      id TEXT PRIMARY KEY NOT NULL,
      economic_transaction_id TEXT NOT NULL,
      document_id TEXT NOT NULL,
      allocated_minor INTEGER NOT NULL,
      currency TEXT NOT NULL,
      created_at TEXT NOT NULL
    );

    CREATE INDEX IF NOT EXISTS idx_alloc_eco ON payment_allocations(economic_transaction_id);

    CREATE TABLE IF NOT EXISTS settings_kv (
      key TEXT PRIMARY KEY NOT NULL,
      value_json TEXT NOT NULL,
      updated_at TEXT NOT NULL
    );

    ALTER TABLE journal_entries ADD COLUMN blocked_unsupported INTEGER NOT NULL DEFAULT 0;
    ALTER TABLE journal_entries ADD COLUMN block_reason TEXT;
    ALTER TABLE journal_entries ADD COLUMN rule_pack_status TEXT;
    "#,
];

pub fn open_db(path: &Path) -> Result<Connection, DbError> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
    Ok(conn)
}

pub fn current_version(conn: &Connection) -> Result<i64, DbError> {
    let exists: Option<i64> = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='schema_migrations'",
            [],
            |r| r.get(0),
        )
        .optional()?;
    if exists.is_none() {
        return Ok(0);
    }
    let v: i64 = conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
        [],
        |r| r.get(0),
    )?;
    Ok(v)
}

pub fn migrate(conn: &Connection) -> Result<i64, DbError> {
    let mut version = current_version(conn)?;
    for (idx, sql) in MIGRATIONS.iter().enumerate() {
        let target = (idx + 1) as i64;
        if version >= target {
            continue;
        }
        conn.execute_batch(sql)
            .map_err(|e| DbError::Migration(format!("v{target}: {e}")))?;
        conn.execute(
            "INSERT INTO schema_migrations(version, applied_at) VALUES (?1, datetime('now'))",
            [target],
        )?;
        version = target;
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn migrations_apply_cleanly() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("t.sqlite");
        let conn = open_db(&path).unwrap();
        let v = migrate(&conn).unwrap();
        assert_eq!(v, MIGRATIONS.len() as i64);
        let v2 = migrate(&conn).unwrap();
        assert_eq!(v2, v);
    }
}
