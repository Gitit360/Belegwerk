//! All mutating/sensitive operations go through the Permission Gate (M02).

use crate::actions::KnownAction;
use crate::approval::ApprovalService;
use crate::audit::{AuditEvent, AuditSink};
use crate::permission::{
    GateError, PermissionDecision, PermissionGate, PermissionRequest, ResourceRef, Scope,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error(transparent)]
    Gate(#[from] GateError),
    #[error("{0}")]
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AppCommand {
    DocumentsImport {
        source_label: String,
        filename: String,
    },
    BankTransactionsRead {
        account_id: String,
    },
    BankPaymentCreate {
        account_id: String,
        amount_minor: i64,
    },
    BookingPost {
        journal_id: String,
    },
    MailSend {
        to: String,
        subject: String,
    },
    ExportBuild {
        format: String,
    },
}

impl AppCommand {
    pub fn known_action(&self) -> KnownAction {
        match self {
            Self::DocumentsImport { .. } => KnownAction::DocumentsImport,
            Self::BankTransactionsRead { .. } => KnownAction::BankTransactionsRead,
            Self::BankPaymentCreate { .. } => KnownAction::BankPaymentCreate,
            Self::BookingPost { .. } => KnownAction::BookingPost,
            Self::MailSend { .. } => KnownAction::MailSend,
            Self::ExportBuild { .. } => KnownAction::ExportBuild,
        }
    }

    pub fn resource(&self) -> ResourceRef {
        match self {
            Self::DocumentsImport { .. } => ResourceRef {
                kind: "document".into(),
                id: None,
            },
            Self::BankTransactionsRead { account_id }
            | Self::BankPaymentCreate { account_id, .. } => ResourceRef {
                kind: "bank_account".into(),
                id: Some(account_id.clone()),
            },
            Self::BookingPost { journal_id } => ResourceRef {
                kind: "journal".into(),
                id: Some(journal_id.clone()),
            },
            Self::MailSend { .. } => ResourceRef {
                kind: "mail".into(),
                id: None,
            },
            Self::ExportBuild { format } => ResourceRef {
                kind: "export".into(),
                id: Some(format.clone()),
            },
        }
    }

    pub fn payload_for_hash(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub ok: bool,
    pub message: String,
}

/// Central dispatcher — UI/CLI/MCP must use this; no bypass API.
pub struct CommandBus<'a> {
    pub gate: &'a PermissionGate,
    pub approvals: &'a ApprovalService,
    pub audit: &'a dyn AuditSink,
    pub caller: String,
}

impl CommandBus<'_> {
    pub fn dispatch(&self, cmd: AppCommand) -> Result<CommandResult, CommandError> {
        let req = PermissionRequest::for_known(
            cmd.known_action(),
            cmd.resource(),
            Scope {
                label: "default".into(),
            },
            &cmd.payload_for_hash(),
            &self.caller,
        );

        let decision = self.gate.decide(&req);
        self.audit.record(AuditEvent {
            at: Utc::now(),
            caller: self.caller.clone(),
            action: req.action.0.clone(),
            decision: format!("{decision:?}"),
            detail: Some(cmd.payload_for_hash()),
        });

        match decision {
            PermissionDecision::Deny { reason } => {
                return Err(CommandError::Gate(GateError::Denied(reason)));
            }
            PermissionDecision::RequireApproval => {
                if self
                    .approvals
                    .find_valid(&req.action, &req.payload_hash, Utc::now())
                    .is_none()
                {
                    return Err(CommandError::Gate(GateError::ApprovalRequired));
                }
            }
            PermissionDecision::Allow => {}
        }

        // Sensitive side effects only after gate. Bank payment never reaches here successfully.
        match cmd {
            AppCommand::BankPaymentCreate { .. } => Err(CommandError::Failed(
                "unreachable: bank payments are permanently blocked".into(),
            )),
            AppCommand::DocumentsImport {
                source_label,
                filename,
            } => Ok(CommandResult {
                ok: true,
                message: format!("authorized import from {source_label}: {filename}"),
            }),
            AppCommand::BankTransactionsRead { account_id } => Ok(CommandResult {
                ok: true,
                message: format!("authorized read for account {account_id}"),
            }),
            AppCommand::BookingPost { journal_id } => Ok(CommandResult {
                ok: true,
                message: format!("authorized booking post for {journal_id}"),
            }),
            AppCommand::MailSend { to, subject } => Ok(CommandResult {
                ok: true,
                message: format!("authorized mail send to {to}: {subject}"),
            }),
            AppCommand::ExportBuild { format } => Ok(CommandResult {
                ok: true,
                message: format!("authorized export build format={format}"),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::InMemoryAuditSink;

    #[test]
    fn bank_payment_command_denied() {
        let gate = PermissionGate::default();
        let approvals = ApprovalService::default();
        let audit = InMemoryAuditSink::default();
        let bus = CommandBus {
            gate: &gate,
            approvals: &approvals,
            audit: &audit,
            caller: "ui".into(),
        };
        let err = bus
            .dispatch(AppCommand::BankPaymentCreate {
                account_id: "x".into(),
                amount_minor: 100,
            })
            .unwrap_err();
        assert!(matches!(err, CommandError::Gate(GateError::Denied(_))));
        assert!(!audit.events().is_empty());
    }

    #[test]
    fn document_import_allowed() {
        let gate = PermissionGate::default();
        let approvals = ApprovalService::default();
        let audit = InMemoryAuditSink::default();
        let bus = CommandBus {
            gate: &gate,
            approvals: &approvals,
            audit: &audit,
            caller: "ui".into(),
        };
        let res = bus
            .dispatch(AppCommand::DocumentsImport {
                source_label: "synthetic".into(),
                filename: "a.txt".into(),
            })
            .unwrap();
        assert!(res.ok);
    }
}
