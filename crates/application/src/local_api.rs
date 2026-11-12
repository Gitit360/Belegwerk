//! Local REST /v1 stub surface (loopback). No network exposure by default.

use belegwerk_domain::reporting::TaxReturnDraft;
use belegwerk_domain::StatusDimensions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub name: String,
    pub version: String,
    pub bank_payments_enabled: bool,
}

pub fn health() -> HealthResponse {
    HealthResponse {
        name: "Belegwerk".into(),
        version: "0.1.0".into(),
        bank_payments_enabled: false,
    }
}

pub fn get_tax_draft_shell(kind: &str, period: &str) -> TaxReturnDraft {
    TaxReturnDraft::shell(kind, period)
}

pub fn blocked_unsupported(message: impl Into<String>) -> ApiErrorBody {
    ApiErrorBody {
        code: "blocked_unsupported".into(),
        message: message.into(),
        blocked: true,
    }
}

pub fn initial_status() -> StatusDimensions {
    StatusDimensions::initial_unmatched()
}

/// Route table documentation helper for OpenAPI alignment.
pub fn v1_routes() -> &'static [(&'static str, &'static str)] {
    &[
        ("GET /v1/health", "Liveness"),
        ("GET /v1/status", "Short German status"),
        ("GET /v1/status-dimensions/initial", "Matching status stub"),
        (
            "GET /v1/tax-drafts/{kind}/{period}",
            "Tax return draft shell",
        ),
        ("POST /v1/imports", "Gated document import"),
        (
            "POST /v1/match/suggestions",
            "Amount/reference match heuristics",
        ),
        (
            "POST /v1/journal/drafts",
            "Balanced journal draft / blocked cases",
        ),
        ("POST /v1/approvals", "Create action approval"),
        ("POST /v1/bank/payments", "Always 403 — not implemented"),
    ]
}
