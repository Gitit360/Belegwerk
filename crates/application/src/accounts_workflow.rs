//! Bank accounts + fixture replay → source + economic transactions (M04/M05 bridge).

use crate::bank_import::{BankImportError, BankImportService};
use crate::macos_probes::live_connectors_enabled;
use crate::moneymoney_live::{self, MoneyMoneyLiveError};
use crate::moneymoney_replay::replay_fixture_dir;
use crate::permission::PermissionGate;
use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AccountsError {
    #[error(transparent)]
    Bank(#[from] BankImportError),
    #[error(transparent)]
    Live(#[from] MoneyMoneyLiveError),
    #[error("{0}")]
    Storage(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRow {
    pub id: String,
    pub provider: String,
    pub label: String,
    pub currency: String,
    pub transaction_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTxRow {
    pub id: String,
    pub source_ref: String,
    pub booking_date: String,
    pub amount_minor: i64,
    pub currency: String,
    pub counterparty: Option<String>,
    pub purpose: Option<String>,
    pub economic_transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsSnapshot {
    pub note: String,
    pub accounts: Vec<AccountRow>,
    pub recent: Vec<AccountTxRow>,
}

pub struct AccountsWorkflow<'a> {
    pub gate: &'a PermissionGate,
    pub paths: DataPaths,
    pub caller: String,
}

impl AccountsWorkflow<'_> {
    fn conn(&self) -> Result<Connection, AccountsError> {
        self.paths
            .ensure()
            .map_err(|e| AccountsError::Storage(e.to_string()))?;
        let conn =
            open_db(&self.paths.db_path()).map_err(|e| AccountsError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| AccountsError::Storage(e.to_string()))?;
        Ok(conn)
    }

    pub fn snapshot(&self) -> Result<AccountsSnapshot, AccountsError> {
        let conn = self.conn()?;
        let accounts = list_accounts(&conn)?;
        let recent = list_recent(&conn, 50)?;
        let note = if live_connectors_enabled() {
            "Live-Connector an — MoneyMoney AppleScript (read-only) oder Fixture-Replay"
        } else {
            "Fixture-Replay Default — Live: BELEGWERK_LIVE_CONNECTORS=1"
        };
        Ok(AccountsSnapshot {
            note: note.into(),
            accounts,
            recent,
        })
    }

    /// Import JSON fixtures from `dir` and promote each source tx to an economic tx (idempotent link).
    pub fn replay_bank_fixtures(&self, dir: &Path) -> Result<usize, AccountsError> {
        let svc = BankImportService {
            gate: self.gate,
            paths: self.paths.clone(),
            caller: self.caller.clone(),
        };
        let imported = replay_fixture_dir(&svc, dir)?;
        let conn = self.conn()?;
        let mut linked = 0usize;
        for tx in &imported {
            if ensure_economic_for_source(&conn, tx)? {
                linked += 1;
            }
        }
        Ok(linked.max(imported.len()))
    }

    pub fn replay_default_synthetic_bank(&self) -> Result<usize, AccountsError> {
        let dir = default_synthetic_bank_dir();
        if !dir.exists() {
            return Err(AccountsError::Storage(format!(
                "fixture dir missing: {}",
                dir.display()
            )));
        }
        self.replay_bank_fixtures(&dir)
    }

    /// Read-only live import from MoneyMoney (`export transactions as "plist"`).
    /// `from_date` is YYYY-MM-DD; default = 30 days ago when `None`.
    pub fn import_live_moneymoney(&self, from_date: Option<&str>) -> Result<usize, AccountsError> {
        let from = from_date.map(str::to_string).unwrap_or_else(|| {
            (chrono::Utc::now().date_naive() - chrono::Duration::days(30))
                .format("%Y-%m-%d")
                .to_string()
        });
        let inputs = moneymoney_live::fetch_transactions_since(&from)?;
        let svc = BankImportService {
            gate: self.gate,
            paths: self.paths.clone(),
            caller: self.caller.clone(),
        };
        let imported = svc.import_many(&inputs)?;
        let conn = self.conn()?;
        for tx in &imported {
            let _ = ensure_economic_for_source(&conn, tx)?;
        }
        Ok(imported.len())
    }
}

fn default_synthetic_bank_dir() -> PathBuf {
    // Prefer repo fixtures relative to CARGO_MANIFEST_DIR of application crate when tests run;
    // desktop passes an absolute path from Tauri.
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/synthetic/bank")
}

fn list_accounts(conn: &Connection) -> Result<Vec<AccountRow>, AccountsError> {
    let mut stmt = conn
        .prepare(
            "SELECT source_account_id, provider_id, currency, COUNT(*)
             FROM source_transactions
             GROUP BY source_account_id, provider_id, currency
             ORDER BY source_account_id",
        )
        .map_err(|e| AccountsError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([], |r| {
            Ok(AccountRow {
                id: r.get(0)?,
                provider: r.get(1)?,
                label: r.get::<_, String>(0)?,
                currency: r.get(2)?,
                transaction_count: r.get(3)?,
            })
        })
        .map_err(|e| AccountsError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(|e| AccountsError::Storage(e.to_string()))?);
    }
    Ok(out)
}

fn list_recent(conn: &Connection, limit: i64) -> Result<Vec<AccountTxRow>, AccountsError> {
    let mut stmt = conn
        .prepare(
            "SELECT s.id, s.source_transaction_ref, s.booking_date, s.amount_minor, s.currency,
                    s.counterparty, s.purpose,
                    (SELECT economic_transaction_id FROM economic_source_links l
                     WHERE l.source_transaction_id = s.id LIMIT 1)
             FROM source_transactions s
             ORDER BY s.booking_date DESC, s.imported_at DESC
             LIMIT ?1",
        )
        .map_err(|e| AccountsError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([limit], |r| {
            Ok(AccountTxRow {
                id: r.get(0)?,
                source_ref: r.get(1)?,
                booking_date: r.get(2)?,
                amount_minor: r.get(3)?,
                currency: r.get(4)?,
                counterparty: r.get(5)?,
                purpose: r.get(6)?,
                economic_transaction_id: r.get(7)?,
            })
        })
        .map_err(|e| AccountsError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(|e| AccountsError::Storage(e.to_string()))?);
    }
    Ok(out)
}

/// Returns true if a new economic row was inserted.
pub fn ensure_economic_for_source(
    conn: &Connection,
    tx: &belegwerk_domain::SourceTransaction,
) -> Result<bool, AccountsError> {
    let src_id = tx.id.to_string();
    let existing: Option<String> = conn
        .query_row(
            "SELECT economic_transaction_id FROM economic_source_links WHERE source_transaction_id = ?1",
            [&src_id],
            |r| r.get(0),
        )
        .optional()
        .map_err(|e| AccountsError::Storage(e.to_string()))?;
    if existing.is_some() {
        return Ok(false);
    }
    let eco_id = Uuid::new_v4().to_string();
    let desc = match (&tx.purpose, &tx.counterparty) {
        (Some(p), Some(c)) => format!("{p} [{c}]"),
        (Some(p), None) => p.clone(),
        (None, Some(c)) => c.clone(),
        _ => tx.source_transaction_ref.clone(),
    };
    conn.execute(
        "INSERT INTO economic_transactions(id, amount_minor, currency, booking_date, description, status)
         VALUES (?1, ?2, ?3, ?4, ?5, 'imported')",
        (
            &eco_id,
            tx.amount.amount_minor,
            &tx.amount.currency.0,
            tx.booking_date.0.to_string(),
            desc,
        ),
    )
    .map_err(|e| AccountsError::Storage(e.to_string()))?;
    conn.execute(
        "INSERT INTO economic_source_links(economic_transaction_id, source_transaction_id) VALUES (?1, ?2)",
        (&eco_id, &src_id),
    )
    .map_err(|e| AccountsError::Storage(e.to_string()))?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn replay_creates_accounts_and_economic() {
        let dir = tempdir().unwrap();
        let gate = PermissionGate::default();
        let wf = AccountsWorkflow {
            gate: &gate,
            paths: DataPaths::new(dir.path()),
            caller: "test".into(),
        };
        let n = wf.replay_default_synthetic_bank().unwrap();
        assert!(n >= 1);
        let snap = wf.snapshot().unwrap();
        assert!(!snap.accounts.is_empty());
        assert!(!snap.recent.is_empty());
        assert!(snap.recent[0].economic_transaction_id.is_some());
    }

    #[test]
    #[ignore = "requires macOS MoneyMoney + BELEGWERK_LIVE_CONNECTORS=1"]
    fn live_moneymoney_e2e() {
        std::env::set_var("BELEGWERK_LIVE_CONNECTORS", "1");
        let dir = tempdir().unwrap();
        let gate = PermissionGate::default();
        let wf = AccountsWorkflow {
            gate: &gate,
            paths: DataPaths::new(dir.path()),
            caller: "live-e2e".into(),
        };
        let from = (chrono::Utc::now().date_naive() - chrono::Duration::days(14))
            .format("%Y-%m-%d")
            .to_string();
        let n = wf.import_live_moneymoney(Some(&from)).expect("live import");
        assert!(n >= 1, "expected at least one booked tx, got {n}");
        let snap = wf.snapshot().unwrap();
        assert!(!snap.recent.is_empty());
    }
}
