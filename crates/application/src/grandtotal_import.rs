//! GrandTotal 10 file-based import stub (M09 / D-02). No proprietary DB writes.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrandTotalError {
    #[error("{0}")]
    Io(String),
    #[error("proprietary DB writes are forbidden (D-02)")]
    ProprietaryDbForbidden,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GrandTotalInvoice {
    pub invoice_number: String,
    pub issue_date: String,
    pub amount_minor: i64,
    pub currency: String,
    pub customer: String,
    pub source: String,
}

#[derive(Debug, Deserialize)]
struct FixtureFile {
    invoices: Vec<GrandTotalInvoice>,
}

pub fn parse_export_json(path: &Path) -> Result<Vec<GrandTotalInvoice>, GrandTotalError> {
    let text = fs::read_to_string(path).map_err(|e| GrandTotalError::Io(e.to_string()))?;
    // Accept either {invoices:[...]} or a bare array.
    if let Ok(file) = serde_json::from_str::<FixtureFile>(&text) {
        return Ok(file.invoices);
    }
    serde_json::from_str(&text).map_err(|e| GrandTotalError::Io(e.to_string()))
}

pub fn write_proprietary_db(_path: &Path) -> Result<(), GrandTotalError> {
    Err(GrandTotalError::ProprietaryDbForbidden)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parses_synthetic_gt_fixture() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/synthetic/invoices/gt10-synth-export.json");
        let invoices = parse_export_json(&path).unwrap();
        assert_eq!(invoices.len(), 1);
        assert_eq!(invoices[0].invoice_number, "GT-SYNTH-2001");
        assert!(write_proprietary_db(Path::new("/tmp/gt.db")).is_err());
    }
}
