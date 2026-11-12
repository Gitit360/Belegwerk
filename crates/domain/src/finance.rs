//! Bank/finance domain stubs — read-only import model.
//! No payment creation APIs exist here (REQ-S-004).

use crate::ids::{EconomicTransactionId, SourceTransactionId};
use crate::money::{CurrencyCode, MoneyAmount};
use crate::time::{CalendarDate, UtcInstant};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Imported,
    Reviewed,
    Linked,
    Ignored,
}

/// Raw imported bank/provider line — not yet an economic event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceTransaction {
    pub id: SourceTransactionId,
    pub provider_id: String,
    pub source_account_id: String,
    pub source_transaction_ref: String,
    pub booking_date: CalendarDate,
    pub value_date: Option<CalendarDate>,
    pub amount: MoneyAmount,
    pub counterparty: Option<String>,
    pub purpose: Option<String>,
    pub status: TransactionStatus,
    pub imported_at: UtcInstant,
    pub raw_ref: Option<String>,
}

impl SourceTransaction {
    pub fn synthetic_example() -> Self {
        Self {
            id: SourceTransactionId::new(),
            provider_id: "synthetic.bank".into(),
            source_account_id: "acct_synth_001".into(),
            source_transaction_ref: "synth-tx-001".into(),
            booking_date: CalendarDate::from_ymd(2026, 1, 15).expect("valid"),
            value_date: CalendarDate::from_ymd(2026, 1, 15),
            amount: MoneyAmount::new(-1999, CurrencyCode::eur()),
            counterparty: Some("Example SaaS GmbH (synthetic)".into()),
            purpose: Some("INV-SYNTH-42".into()),
            status: TransactionStatus::Imported,
            imported_at: Utc::now(),
            raw_ref: Some("fixture:bank/synth-tx-001.json".into()),
        }
    }
}

/// Normalized economic event after dedup/merge review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicTransaction {
    pub id: EconomicTransactionId,
    pub amount: MoneyAmount,
    pub booking_date: CalendarDate,
    pub description: Option<String>,
    /// Linked source transaction ids (many-to-many later via junction table).
    pub source_transaction_ids: Vec<SourceTransactionId>,
    pub status: TransactionStatus,
}

impl EconomicTransaction {
    pub fn from_single_source(source: &SourceTransaction) -> Self {
        Self {
            id: EconomicTransactionId::new(),
            amount: source.amount.clone(),
            booking_date: source.booking_date,
            description: source.purpose.clone(),
            source_transaction_ids: vec![source.id],
            status: TransactionStatus::Imported,
        }
    }
}
