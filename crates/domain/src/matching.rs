//! Matching stubs — types and status dimensions only (M05 later).

use crate::ids::{DocumentId, EconomicTransactionId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchConfidence {
    Exact,
    High,
    Medium,
    Low,
    Manual,
}

/// Five status dimensions from Product Description §10.2 (stubs).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusDimensions {
    pub document_completeness: DocumentCompleteness,
    pub match_state: MatchState,
    pub extraction_state: ExtractionState,
    pub booking_state: BookingState,
    pub export_state: ExportState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentCompleteness {
    Missing,
    Placeholder,
    Partial,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchState {
    Unmatched,
    Suggested,
    Linked,
    Conflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtractionState {
    NotStarted,
    Partial,
    ReadyForReview,
    Accepted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookingState {
    Draft,
    Approved,
    Posted,
    BlockedUnsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportState {
    NotReady,
    Ready,
    Exported,
}

impl StatusDimensions {
    pub fn initial_unmatched() -> Self {
        Self {
            document_completeness: DocumentCompleteness::Missing,
            match_state: MatchState::Unmatched,
            extraction_state: ExtractionState::NotStarted,
            booking_state: BookingState::Draft,
            export_state: ExportState::NotReady,
        }
    }

    /// Unsupported tax/accounting cases must surface as blocked — never silent map.
    pub fn blocked_unsupported() -> Self {
        Self {
            booking_state: BookingState::BlockedUnsupported,
            ..Self::initial_unmatched()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchCandidate {
    pub economic_transaction_id: EconomicTransactionId,
    pub document_id: DocumentId,
    pub confidence: MatchConfidence,
    pub reason: String,
}
