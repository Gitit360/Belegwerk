//! MoneyMoney adapter fixture replay (M04). No live AppleScript on Linux.

use crate::bank_import::{BankImportError, BankImportService, SourceTransactionInput};
use belegwerk_domain::SourceTransaction;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureFile {
    provider_id: String,
    source_account_id: String,
    source_transaction_ref: String,
    booking_date: String,
    value_date: Option<String>,
    amount_minor: i64,
    currency: String,
    counterparty: Option<String>,
    purpose: Option<String>,
    #[serde(default)]
    note: Option<String>,
}

pub fn load_fixture_file(path: &Path) -> Result<SourceTransactionInput, BankImportError> {
    let text = fs::read_to_string(path).map_err(|e| BankImportError::Storage(e.to_string()))?;
    let f: FixtureFile =
        serde_json::from_str(&text).map_err(|e| BankImportError::Storage(e.to_string()))?;
    let _ = f.note; // synthetic marker only
    Ok(SourceTransactionInput {
        provider_id: f.provider_id,
        source_account_id: f.source_account_id,
        source_transaction_ref: f.source_transaction_ref,
        booking_date: f.booking_date,
        value_date: f.value_date,
        amount_minor: f.amount_minor,
        currency: f.currency,
        counterparty: f.counterparty,
        purpose: f.purpose,
        raw_ref: Some(path.display().to_string()),
    })
}

pub fn replay_fixture_dir(
    svc: &BankImportService<'_>,
    dir: &Path,
) -> Result<Vec<SourceTransaction>, BankImportError> {
    let mut inputs = Vec::new();
    let entries = fs::read_dir(dir).map_err(|e| BankImportError::Storage(e.to_string()))?;
    for entry in entries {
        let entry = entry.map_err(|e| BankImportError::Storage(e.to_string()))?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            inputs.push(load_fixture_file(&path)?);
        }
    }
    inputs.sort_by(|a, b| a.source_transaction_ref.cmp(&b.source_transaction_ref));
    svc.import_many(&inputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permission::PermissionGate;
    use belegwerk_infrastructure::DataPaths;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn replays_repo_synthetic_bank_fixture() {
        let fixture =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/synthetic/bank");
        assert!(
            fixture.exists(),
            "fixture dir missing: {}",
            fixture.display()
        );
        let dir = tempdir().unwrap();
        let gate = PermissionGate::default();
        let svc = BankImportService {
            gate: &gate,
            paths: DataPaths::new(dir.path()),
            caller: "moneymoney-replay".into(),
        };
        let txs = replay_fixture_dir(&svc, &fixture).unwrap();
        assert!(!txs.is_empty());
        assert_eq!(txs[0].provider_id, "moneymoney");
        assert_eq!(txs[0].amount.amount_minor, -2499);
    }
}
