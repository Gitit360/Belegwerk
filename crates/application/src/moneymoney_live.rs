//! Read-only MoneyMoney AppleScript import (macOS + BELEGWERK_LIVE_CONNECTORS).
//! Never calls payment/transfer commands.

use crate::bank_import::SourceTransactionInput;
use crate::macos_probes::live_connectors_enabled;
use serde::Deserialize;
#[cfg(target_os = "macos")]
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoneyMoneyLiveError {
    #[error("live connectors disabled — set BELEGWERK_LIVE_CONNECTORS=1")]
    LiveDisabled,
    #[error("MoneyMoney live import requires macOS")]
    NotMacos,
    #[error("{0}")]
    Osascript(String),
    #[error("plist parse: {0}")]
    Plist(String),
}

#[derive(Debug, Deserialize)]
struct TxExport {
    #[serde(default)]
    transactions: Vec<MmTx>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MmTx {
    id: Option<i64>,
    account_uuid: Option<String>,
    account_number: Option<String>,
    amount: Option<f64>,
    currency: Option<String>,
    booked: Option<bool>,
    booking_date: Option<plist::Date>,
    value_date: Option<plist::Date>,
    name: Option<String>,
    purpose: Option<String>,
    end_to_end_reference: Option<String>,
}

/// Export booked transactions since `from_date` (YYYY-MM-DD) via AppleScript `export transactions … as "plist"`.
pub fn fetch_transactions_since(
    from_date: &str,
) -> Result<Vec<SourceTransactionInput>, MoneyMoneyLiveError> {
    if !live_connectors_enabled() {
        return Err(MoneyMoneyLiveError::LiveDisabled);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = from_date;
        Err(MoneyMoneyLiveError::NotMacos)
    }
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            r#"tell application "MoneyMoney" to export transactions from date "{from_date}" as "plist""#
        );
        let output = Command::new("osascript")
            .args(["-e", &script])
            .output()
            .map_err(|e| MoneyMoneyLiveError::Osascript(e.to_string()))?;
        if !output.status.success() {
            return Err(MoneyMoneyLiveError::Osascript(format!(
                "exit {:?} stderr={}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        parse_transaction_plist(&output.stdout)
    }
}

pub fn parse_transaction_plist(
    bytes: &[u8],
) -> Result<Vec<SourceTransactionInput>, MoneyMoneyLiveError> {
    let export: TxExport =
        plist::from_bytes(bytes).map_err(|e| MoneyMoneyLiveError::Plist(e.to_string()))?;
    let mut out = Vec::new();
    for tx in export.transactions {
        if tx.booked != Some(true) {
            continue;
        }
        let Some(id) = tx.id else { continue };
        let amount = tx.amount.unwrap_or(0.0);
        let amount_minor = (amount * 100.0).round() as i64;
        let currency = tx.currency.unwrap_or_else(|| "EUR".into());
        let account = tx
            .account_uuid
            .or(tx.account_number)
            .unwrap_or_else(|| "unknown".into());
        let booking = tx
            .booking_date
            .map(plist_date_ymd)
            .unwrap_or_else(|| "1970-01-01".into());
        let value = tx.value_date.map(plist_date_ymd);
        let purpose = tx.purpose.or(tx.end_to_end_reference);
        out.push(SourceTransactionInput {
            provider_id: "moneymoney".into(),
            source_account_id: account,
            source_transaction_ref: format!("mm-live-{id}"),
            booking_date: booking,
            value_date: value,
            amount_minor,
            currency,
            counterparty: tx.name,
            purpose,
            raw_ref: Some(format!("moneymoney:appleScript:exportTransactions:{id}")),
        });
    }
    Ok(out)
}

fn plist_date_ymd(d: plist::Date) -> String {
    let xml = d.to_xml_format(); // e.g. 2026-01-15T12:00:00Z
    xml.get(..10).unwrap_or("1970-01-01").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_plist_xml() {
        let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>transactions</key>
  <array>
    <dict>
      <key>id</key><integer>42</integer>
      <key>accountUuid</key><string>acc-1</string>
      <key>amount</key><real>-12.34</real>
      <key>currency</key><string>EUR</string>
      <key>booked</key><true/>
      <key>bookingDate</key><date>2026-01-15T12:00:00Z</date>
      <key>name</key><string>Vendor</string>
      <key>purpose</key><string>INV-1</string>
    </dict>
    <dict>
      <key>id</key><integer>43</integer>
      <key>booked</key><false/>
      <key>amount</key><real>1</real>
    </dict>
  </array>
</dict>
</plist>"#;
        let rows = parse_transaction_plist(xml).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_transaction_ref, "mm-live-42");
        assert_eq!(rows[0].amount_minor, -1234);
        assert_eq!(rows[0].source_account_id, "acc-1");
        assert_eq!(rows[0].counterparty.as_deref(), Some("Vendor"));
    }

    #[test]
    fn refuses_without_live_flag() {
        // Ensure env is off for this process test.
        std::env::remove_var("BELEGWERK_LIVE_CONNECTORS");
        let err = fetch_transactions_since("2026-01-01").unwrap_err();
        assert!(matches!(
            err,
            MoneyMoneyLiveError::LiveDisabled | MoneyMoneyLiveError::NotMacos
        ));
    }
}

#[cfg(all(test, target_os = "macos"))]
mod live_file_smoke {
    use super::*;
    #[test]
    fn parses_tmp_export_if_present() {
        let path = std::path::Path::new("/tmp/mm-tx.plist");
        if !path.exists() {
            return;
        }
        let bytes = std::fs::read(path).unwrap();
        let rows = parse_transaction_plist(&bytes).unwrap();
        assert!(!rows.is_empty());
        assert!(rows.iter().all(|r| r.provider_id == "moneymoney"));
    }
}
