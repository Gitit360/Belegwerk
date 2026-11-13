//! Persist SourceTransaction rows from adapter DTOs (M04). Live MoneyMoney is macOS-only.

use crate::actions::KnownAction;
use crate::permission::{PermissionGate, PermissionRequest, ResourceRef, Scope};
use belegwerk_domain::time::CalendarDate;
use belegwerk_domain::{
    CurrencyCode, MoneyAmount, SourceTransaction, SourceTransactionId, TransactionStatus,
};
use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use chrono::{NaiveDate, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BankImportError {
    #[error("permission denied: {0}")]
    Denied(String),
    #[error("{0}")]
    Storage(String),
    #[error("invalid date: {0}")]
    BadDate(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceTransactionInput {
    pub provider_id: String,
    pub source_account_id: String,
    pub source_transaction_ref: String,
    pub booking_date: String,
    pub value_date: Option<String>,
    pub amount_minor: i64,
    pub currency: String,
    pub counterparty: Option<String>,
    pub purpose: Option<String>,
    pub raw_ref: Option<String>,
}

pub struct BankImportService<'a> {
    pub gate: &'a PermissionGate,
    pub paths: DataPaths,
    pub caller: String,
}

impl BankImportService<'_> {
    pub fn import_many(
        &self,
        inputs: &[SourceTransactionInput],
    ) -> Result<Vec<SourceTransaction>, BankImportError> {
        let req = PermissionRequest::for_known(
            KnownAction::BankTransactionsRead,
            ResourceRef {
                kind: "bank".into(),
                id: None,
            },
            Scope {
                label: "bank-import".into(),
            },
            &format!("count={}", inputs.len()),
            &self.caller,
        );
        self.gate
            .authorize(&req)
            .map_err(|e| BankImportError::Denied(e.to_string()))?;

        self.paths
            .ensure()
            .map_err(|e| BankImportError::Storage(e.to_string()))?;
        let conn =
            open_db(&self.paths.db_path()).map_err(|e| BankImportError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| BankImportError::Storage(e.to_string()))?;

        let mut out = Vec::new();
        for input in inputs {
            let tx = to_domain(input)?;
            upsert_source(&conn, &tx).map_err(|e| BankImportError::Storage(e.to_string()))?;
            out.push(tx);
        }
        Ok(out)
    }
}

fn parse_date(s: &str) -> Result<CalendarDate, BankImportError> {
    let d = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| BankImportError::BadDate(format!("{s}: {e}")))?;
    Ok(CalendarDate(d))
}

fn to_domain(input: &SourceTransactionInput) -> Result<SourceTransaction, BankImportError> {
    Ok(SourceTransaction {
        id: SourceTransactionId::new(),
        provider_id: input.provider_id.clone(),
        source_account_id: input.source_account_id.clone(),
        source_transaction_ref: input.source_transaction_ref.clone(),
        booking_date: parse_date(&input.booking_date)?,
        value_date: match &input.value_date {
            Some(v) => Some(parse_date(v)?),
            None => None,
        },
        amount: MoneyAmount::new(input.amount_minor, CurrencyCode(input.currency.clone())),
        counterparty: input.counterparty.clone(),
        purpose: input.purpose.clone(),
        status: TransactionStatus::Imported,
        imported_at: Utc::now(),
        raw_ref: input.raw_ref.clone(),
    })
}

fn upsert_source(conn: &Connection, tx: &SourceTransaction) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO source_transactions(
            id, provider_id, source_account_id, source_transaction_ref, booking_date, value_date,
            amount_minor, currency, counterparty, purpose, status, imported_at, raw_ref
         ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)
         ON CONFLICT(provider_id, source_account_id, source_transaction_ref) DO UPDATE SET
            booking_date=excluded.booking_date,
            value_date=excluded.value_date,
            amount_minor=excluded.amount_minor,
            currency=excluded.currency,
            counterparty=excluded.counterparty,
            purpose=excluded.purpose,
            status=excluded.status,
            imported_at=excluded.imported_at,
            raw_ref=excluded.raw_ref",
        (
            tx.id.to_string(),
            &tx.provider_id,
            &tx.source_account_id,
            &tx.source_transaction_ref,
            tx.booking_date.0.to_string(),
            tx.value_date.map(|d| d.0.to_string()),
            tx.amount.amount_minor,
            &tx.amount.currency.0,
            &tx.counterparty,
            &tx.purpose,
            "imported",
            tx.imported_at.to_rfc3339(),
            &tx.raw_ref,
        ),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn imports_synthetic_fixture_idempotently() {
        let dir = tempdir().unwrap();
        let gate = PermissionGate::default();
        let svc = BankImportService {
            gate: &gate,
            paths: DataPaths::new(dir.path()),
            caller: "test".into(),
        };
        let input = SourceTransactionInput {
            provider_id: "moneymoney".into(),
            source_account_id: "synth-gls-main".into(),
            source_transaction_ref: "mm-synth-001".into(),
            booking_date: "2026-01-15".into(),
            value_date: Some("2026-01-15".into()),
            amount_minor: -2499,
            currency: "EUR".into(),
            counterparty: Some("Synthetic Cloud Tools LLC".into()),
            purpose: Some("INV-SYNTH-1001 SaaS".into()),
            raw_ref: Some("tests/fixtures/synthetic/bank/mm-synth-001.json".into()),
        };
        assert_eq!(svc.import_many(std::slice::from_ref(&input)).unwrap().len(), 1);
        assert_eq!(svc.import_many(&[input]).unwrap().len(), 1);
        let conn = open_db(&DataPaths::new(dir.path()).db_path()).unwrap();
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM source_transactions", [], |r| r.get(0))
            .unwrap();
        assert_eq!(n, 1);
    }
}
