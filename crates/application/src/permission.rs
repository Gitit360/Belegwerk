use crate::actions::KnownAction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRef {
    pub kind: String,
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scope {
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequest {
    pub action: ActionId,
    pub resource: ResourceRef,
    pub scope: Scope,
    pub payload_hash: String,
    pub caller: String,
}

impl PermissionRequest {
    pub fn for_known(
        action: KnownAction,
        resource: ResourceRef,
        scope: Scope,
        payload: &str,
        caller: impl Into<String>,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        let payload_hash = hex::encode(hasher.finalize());
        Self {
            action: ActionId(action.as_str().into()),
            resource,
            scope,
            payload_hash,
            caller: caller.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionDecision {
    Allow,
    Deny { reason: String },
    RequireApproval,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GateError {
    #[error("permission denied: {0}")]
    Denied(String),
    #[error("approval required for action")]
    ApprovalRequired,
}

/// Core permission gate — UI/CLI/MCP/plugins must call this; there is no bypass path.
#[derive(Debug, Default, Clone)]
pub struct PermissionGate {
    /// Granted action ids for the current session/profile (stub allowlist).
    granted: Vec<String>,
}

impl PermissionGate {
    pub fn new_with_grants(actions: impl IntoIterator<Item = KnownAction>) -> Self {
        Self {
            granted: actions
                .into_iter()
                .map(|a| a.as_str().to_string())
                .collect(),
        }
    }

    pub fn decide(&self, req: &PermissionRequest) -> PermissionDecision {
        if req.action.0 == KnownAction::BankPaymentCreate.as_str() {
            return PermissionDecision::Deny {
                reason: "bank payments are not implemented and must never be enabled via plugins"
                    .into(),
            };
        }

        let known = match req.action.0.as_str() {
            "mail.send" => Some(KnownAction::MailSend),
            "booking.post" => Some(KnownAction::BookingPost),
            "tax.submit" => Some(KnownAction::TaxSubmit),
            "delivery.submit" => Some(KnownAction::DeliverySubmit),
            "plugin.install" => Some(KnownAction::PluginInstall),
            _ => None,
        };

        if let Some(a) = known {
            if a.requires_approval() {
                if !self.granted.iter().any(|g| g == a.as_str()) {
                    // Still require approval even if conceptually "granted" as capability.
                }
                return PermissionDecision::RequireApproval;
            }
        }

        if self.granted.iter().any(|g| g == &req.action.0)
            || matches!(
                req.action.0.as_str(),
                "documents.read"
                    | "documents.import"
                    | "bank.transactions.read"
                    | "mail.search"
                    | "mail.draft.create"
                    | "export.build"
                    | "match.confirm"
                    | "match.reject"
                    | "reminder.escalate"
                    | "reminder.complete"
                    | "settings.write"
                    | "journal.draft.save"
            )
        {
            PermissionDecision::Allow
        } else {
            PermissionDecision::Deny {
                reason: format!("action not granted: {}", req.action.0),
            }
        }
    }

    pub fn authorize(&self, req: &PermissionRequest) -> Result<(), GateError> {
        match self.decide(req) {
            PermissionDecision::Allow => Ok(()),
            PermissionDecision::Deny { reason } => Err(GateError::Denied(reason)),
            PermissionDecision::RequireApproval => Err(GateError::ApprovalRequired),
        }
    }

    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank_payment_always_denied() {
        let gate = PermissionGate::new_with_grants([KnownAction::BankPaymentCreate]);
        let req = PermissionRequest::for_known(
            KnownAction::BankPaymentCreate,
            ResourceRef {
                kind: "account".into(),
                id: Some("x".into()),
            },
            Scope {
                label: "default".into(),
            },
            "{}",
            "ui",
        );
        assert!(matches!(gate.decide(&req), PermissionDecision::Deny { .. }));
    }

    #[test]
    fn mail_send_requires_approval() {
        let gate = PermissionGate::default();
        let req = PermissionRequest::for_known(
            KnownAction::MailSend,
            ResourceRef {
                kind: "mail".into(),
                id: None,
            },
            Scope {
                label: "default".into(),
            },
            "to=a@b.c",
            "ui",
        );
        assert_eq!(gate.decide(&req), PermissionDecision::RequireApproval);
    }
}
