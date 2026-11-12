use belegwerk_domain::{
    DocumentId, EconomicTransactionId, MatchCandidate, MatchConfidence, StatusDimensions,
};

pub fn suggest_manual_link(
    economic_transaction_id: EconomicTransactionId,
    document_id: DocumentId,
    reason: impl Into<String>,
) -> MatchCandidate {
    MatchCandidate {
        economic_transaction_id,
        document_id,
        confidence: MatchConfidence::Manual,
        reason: reason.into(),
    }
}

pub fn status_for_unsupported_case() -> StatusDimensions {
    StatusDimensions::blocked_unsupported()
}
