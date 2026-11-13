//! Belegwerk domain crate — deterministic types only.
//! No tax rule evaluation, no AI network I/O, no payments.

pub mod accounting;
pub mod ai;
pub mod document;
pub mod export;
pub mod extraction;
pub mod finance;
pub mod ids;
pub mod mail;
pub mod matching;
pub mod money;
pub mod reminders;
pub mod reporting;
pub mod settings;
pub mod time;

pub use document::{ContentHash, DocumentMeta, ImportBatch, ObjectKey};
pub use finance::{EconomicTransaction, SourceTransaction, TransactionStatus};
pub use ids::{DocumentId, EconomicTransactionId, ImportBatchId, SourceTransactionId};
pub use matching::{BatchPreview, MatchCandidate, MatchConfidence, MatchDecision, MatchDecisionKind, PaymentAllocation, StatusDimensions};
pub use settings::{ReportingCadence, TaxProfileSettings, VatMethod};
pub use money::{CurrencyCode, MoneyAmount};
pub use time::CalendarDate;
