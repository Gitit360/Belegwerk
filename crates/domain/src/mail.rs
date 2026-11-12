//! Mail / portal stubs (M10). No silent mutations.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailSearchQuery {
    pub subject_contains: Option<String>,
    pub from_contains: Option<String>,
    pub since: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailHit {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub received_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailDraft {
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalConnectorSlot {
    pub id: String,
    pub display_name: String,
    pub status: String,
}

/// Placeholder slots — no fake brand for mobile (D-07).
pub fn portal_slots_v1() -> Vec<PortalConnectorSlot> {
    vec![
        PortalConnectorSlot {
            id: "mobile-de".into(),
            display_name: "Mobilfunk (Slot)".into(),
            status: "unconfigured".into(),
        },
        PortalConnectorSlot {
            id: "generic-billing".into(),
            display_name: "Generic Billing Portal".into(),
            status: "stub".into(),
        },
    ]
}
