use crate::ids::EconomicTransactionId;
use crate::time::UtcInstant;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingDocumentCase {
    pub id: Uuid,
    pub economic_transaction_id: Option<EconomicTransactionId>,
    pub status: MissingStatus,
    pub escalation_level: u8,
    pub note: Option<String>,
    pub created_at: UtcInstant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingStatus {
    Open,
    WaitingOnMail,
    WaitingOnPortal,
    Resolved,
    Dismissed,
}

impl MissingDocumentCase {
    pub fn open(note: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            economic_transaction_id: None,
            status: MissingStatus::Open,
            escalation_level: 0,
            note: Some(note.into()),
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPlaceholder {
    pub id: Uuid,
    pub case_id: Option<Uuid>,
    pub label: String,
    pub created_at: UtcInstant,
}

impl DocumentPlaceholder {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            case_id: None,
            label: label.into(),
            created_at: Utc::now(),
        }
    }
}
