use crate::permission::{ActionId, ResourceRef};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionApproval {
    pub id: Uuid,
    pub action: ActionId,
    pub resource: ResourceRef,
    pub payload_hash: String,
    pub approved_by: String,
    pub approved_at: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone)]
pub struct ApprovalService {
    approvals: Vec<ActionApproval>,
}

impl ApprovalService {
    pub fn approve(
        &mut self,
        action: ActionId,
        resource: ResourceRef,
        payload_hash: impl Into<String>,
        approved_by: impl Into<String>,
        valid_until: Option<DateTime<Utc>>,
    ) -> ActionApproval {
        let approval = ActionApproval {
            id: Uuid::new_v4(),
            action,
            resource,
            payload_hash: payload_hash.into(),
            approved_by: approved_by.into(),
            approved_at: Utc::now(),
            valid_until,
        };
        self.approvals.push(approval.clone());
        approval
    }

    pub fn find_valid(
        &self,
        action: &ActionId,
        payload_hash: &str,
        at: DateTime<Utc>,
    ) -> Option<&ActionApproval> {
        self.approvals.iter().rev().find(|a| {
            &a.action == action
                && a.payload_hash == payload_hash
                && a.valid_until.map(|u| u >= at).unwrap_or(true)
        })
    }
}
