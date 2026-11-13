//! Persist journal drafts and approve only via Permission Gate (M11).

use crate::actions::KnownAction;
use crate::approval::ApprovalService;
use crate::audit::AuditSink;
use crate::commands::{AppCommand, CommandBus};
use crate::journal::{can_post, JournalError};
use crate::permission::PermissionGate;
use belegwerk_domain::accounting::{JournalEntryDraft, JournalLine, TaxRulePackRef};
use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use chrono::Utc;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JournalStoreError {
    #[error(transparent)]
    Journal(#[from] JournalError),
    #[error("permission: {0}")]
    Gate(String),
    #[error("{0}")]
    Storage(String),
    #[error("{0}")]
    NotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalDraftRow {
    pub id: String,
    pub status: String,
    pub description: Option<String>,
    pub blocked_unsupported: bool,
    pub block_reason: Option<String>,
    pub rule_pack_id: Option<String>,
    pub rule_pack_version: Option<String>,
    pub rule_pack_status: Option<String>,
    pub lines: Vec<JournalLine>,
}

pub struct JournalStore<'a> {
    pub gate: &'a PermissionGate,
    pub paths: DataPaths,
    pub caller: String,
}

impl JournalStore<'_> {
    fn conn(&self) -> Result<Connection, JournalStoreError> {
        self.paths
            .ensure()
            .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
        let conn = open_db(&self.paths.db_path())
            .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| JournalStoreError::Storage(e.to_string()))?;
        Ok(conn)
    }

    pub fn seed_demo_drafts_if_empty(&self) -> Result<(), JournalStoreError> {
        let conn = self.conn()?;
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM journal_entries", [], |r| r.get(0))
            .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
        if n > 0 {
            return Ok(());
        }
        let reviewed = TaxRulePackRef {
            id: "belegwerk-core".into(),
            version: "2026.1.0".into(),
            status: "reviewed".into(),
            source_note: Some("docs/accounting/TAX_RULE_REGISTER_v2026.md".into()),
        };
        let ok = crate::journal::draft_expense_against_bank(
            "j-synth-ok",
            "4950",
            "1200",
            2499,
            "EUR",
            reviewed,
        )?;
        self.save_draft_inner(&conn, &ok)?;
        let blocked = crate::journal::block_unverified_tax_case(
            "j-synth-eu",
            "EU-B2B sale outside v1 matrix (D-06) — keine erfundenen Steuerregeln",
        );
        self.save_draft_inner(&conn, &blocked)?;
        Ok(())
    }

    pub fn save_draft(&self, draft: &JournalEntryDraft) -> Result<(), JournalStoreError> {
        let req = crate::permission::PermissionRequest::for_known(
            KnownAction::JournalDraftSave,
            crate::permission::ResourceRef {
                kind: "journal".into(),
                id: Some(draft.id.clone()),
            },
            crate::permission::Scope {
                label: "journal".into(),
            },
            &draft.id,
            &self.caller,
        );
        self.gate
            .authorize(&req)
            .map_err(|e| JournalStoreError::Gate(e.to_string()))?;
        if !draft.blocked_unsupported {
            crate::journal::assert_balanced(&draft.lines)?;
        }
        let conn = self.conn()?;
        self.save_draft_inner(&conn, draft)
    }

    fn save_draft_inner(
        &self,
        conn: &Connection,
        draft: &JournalEntryDraft,
    ) -> Result<(), JournalStoreError> {
        let status = if draft.blocked_unsupported {
            "blocked"
        } else {
            "draft"
        };
        conn.execute(
            "INSERT INTO journal_entries(id, booked_at, status, description, rule_pack_id, rule_pack_version, blocked_unsupported, block_reason, rule_pack_status)
             VALUES (?1, NULL, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(id) DO UPDATE SET
               status=excluded.status,
               description=excluded.description,
               rule_pack_id=excluded.rule_pack_id,
               rule_pack_version=excluded.rule_pack_version,
               blocked_unsupported=excluded.blocked_unsupported,
               block_reason=excluded.block_reason,
               rule_pack_status=excluded.rule_pack_status",
            (
                &draft.id,
                status,
                &draft.description,
                draft.rule_pack.as_ref().map(|r| r.id.clone()),
                draft.rule_pack.as_ref().map(|r| r.version.clone()),
                draft.blocked_unsupported as i64,
                &draft.block_reason,
                draft.rule_pack.as_ref().map(|r| r.status.clone()),
            ),
        )
        .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
        conn.execute(
            "DELETE FROM journal_lines WHERE journal_entry_id = ?1",
            [&draft.id],
        )
        .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
        for line in &draft.lines {
            conn.execute(
                "INSERT INTO journal_lines(journal_entry_id, account_code, debit_minor, credit_minor, currency)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                (
                    &draft.id,
                    &line.account_code,
                    line.debit_minor,
                    line.credit_minor,
                    &line.currency,
                ),
            )
            .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
        }
        Ok(())
    }

    pub fn list_drafts(&self) -> Result<Vec<JournalDraftRow>, JournalStoreError> {
        self.seed_demo_drafts_if_empty()?;
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                "SELECT id, status, description, blocked_unsupported, block_reason, rule_pack_id, rule_pack_version, rule_pack_status
                 FROM journal_entries ORDER BY id",
            )
            .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
        let rows = stmt
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, Option<String>>(2)?,
                    r.get::<_, i64>(3)?,
                    r.get::<_, Option<String>>(4)?,
                    r.get::<_, Option<String>>(5)?,
                    r.get::<_, Option<String>>(6)?,
                    r.get::<_, Option<String>>(7)?,
                ))
            })
            .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            let (id, status, description, blocked, block_reason, rpid, rpver, rpst) =
                row.map_err(|e| JournalStoreError::Storage(e.to_string()))?;
            let lines = load_lines(&conn, &id)?;
            out.push(JournalDraftRow {
                id,
                status,
                description,
                blocked_unsupported: blocked != 0,
                block_reason,
                rule_pack_id: rpid,
                rule_pack_version: rpver,
                rule_pack_status: rpst,
                lines,
            });
        }
        Ok(out)
    }

    /// Approve/post path: must pass CommandBus BookingPost (RequireApproval unless valid approval).
    pub fn approve_draft(
        &self,
        draft_id: &str,
        approvals: &ApprovalService,
        audit: &dyn AuditSink,
    ) -> Result<String, JournalStoreError> {
        let conn = self.conn()?;
        let draft = load_draft(&conn, draft_id)?;
        can_post(&draft)?;
        let bus = CommandBus {
            gate: self.gate,
            approvals,
            audit,
            caller: self.caller.clone(),
        };
        let result = bus
            .dispatch(AppCommand::BookingPost {
                journal_id: draft_id.to_string(),
            })
            .map_err(|e| JournalStoreError::Gate(e.to_string()))?;
        conn.execute(
            "UPDATE journal_entries SET status = 'approved', booked_at = ?1 WHERE id = ?2",
            (Utc::now().to_rfc3339(), draft_id),
        )
        .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
        Ok(result.message)
    }
}

fn load_lines(conn: &Connection, id: &str) -> Result<Vec<JournalLine>, JournalStoreError> {
    let mut stmt = conn
        .prepare(
            "SELECT account_code, debit_minor, credit_minor, currency FROM journal_lines WHERE journal_entry_id = ?1",
        )
        .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([id], |r| {
            Ok(JournalLine {
                account_code: r.get(0)?,
                debit_minor: r.get(1)?,
                credit_minor: r.get(2)?,
                currency: r.get(3)?,
            })
        })
        .map_err(|e| JournalStoreError::Storage(e.to_string()))?;
    let mut lines = Vec::new();
    for row in rows {
        lines.push(row.map_err(|e| JournalStoreError::Storage(e.to_string()))?);
    }
    Ok(lines)
}

fn load_draft(conn: &Connection, id: &str) -> Result<JournalEntryDraft, JournalStoreError> {
    let row = conn
        .query_row(
            "SELECT description, blocked_unsupported, block_reason, rule_pack_id, rule_pack_version, rule_pack_status
             FROM journal_entries WHERE id = ?1",
            [id],
            |r| {
                Ok((
                    r.get::<_, Option<String>>(0)?,
                    r.get::<_, i64>(1)?,
                    r.get::<_, Option<String>>(2)?,
                    r.get::<_, Option<String>>(3)?,
                    r.get::<_, Option<String>>(4)?,
                    r.get::<_, Option<String>>(5)?,
                ))
            },
        )
        .map_err(|_| JournalStoreError::NotFound(id.into()))?;
    let (description, blocked, block_reason, rpid, rpver, rpst) = row;
    let rule_pack = match (rpid, rpver, rpst) {
        (Some(id), Some(version), Some(status)) => Some(TaxRulePackRef {
            id,
            version,
            status,
            source_note: None,
        }),
        _ => None,
    };
    Ok(JournalEntryDraft {
        id: id.into(),
        description,
        lines: load_lines(conn, id)?,
        rule_pack,
        blocked_unsupported: blocked != 0,
        block_reason,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::InMemoryAuditSink;
    use crate::permission::{ActionId, PermissionRequest, ResourceRef, Scope};
    use sha2::{Digest, Sha256};
    use tempfile::tempdir;

    #[test]
    fn lists_blocked_and_refuses_approve_without_approval() {
        let dir = tempdir().unwrap();
        let gate = PermissionGate::default();
        let store = JournalStore {
            gate: &gate,
            paths: DataPaths::new(dir.path()),
            caller: "test".into(),
        };
        let rows = store.list_drafts().unwrap();
        assert!(rows.iter().any(|r| r.blocked_unsupported));
        let approvals = ApprovalService::default();
        let audit = InMemoryAuditSink::default();
        let err = store
            .approve_draft("j-synth-ok", &approvals, &audit)
            .unwrap_err();
        assert!(matches!(err, JournalStoreError::Gate(_)));
    }

    #[test]
    fn approve_with_valid_approval_marks_approved() {
        let dir = tempdir().unwrap();
        let gate = PermissionGate::default();
        let store = JournalStore {
            gate: &gate,
            paths: DataPaths::new(dir.path()),
            caller: "test".into(),
        };
        store.list_drafts().unwrap();
        let cmd = AppCommand::BookingPost {
            journal_id: "j-synth-ok".into(),
        };
        let mut hasher = Sha256::new();
        hasher.update(cmd.payload_for_hash().as_bytes());
        let payload_hash = hex::encode(hasher.finalize());
        let mut approvals = ApprovalService::default();
        approvals.approve(
            ActionId(KnownAction::BookingPost.as_str().into()),
            ResourceRef {
                kind: "journal".into(),
                id: Some("j-synth-ok".into()),
            },
            payload_hash,
            "tester",
            Some(Utc::now() + chrono::Duration::hours(1)),
        );
        let audit = InMemoryAuditSink::default();
        store
            .approve_draft("j-synth-ok", &approvals, &audit)
            .unwrap();
        let row = store
            .list_drafts()
            .unwrap()
            .into_iter()
            .find(|r| r.id == "j-synth-ok")
            .unwrap();
        assert_eq!(row.status, "approved");
        let _ = PermissionRequest::for_known(
            KnownAction::BookingPost,
            ResourceRef {
                kind: "journal".into(),
                id: Some("j-synth-ok".into()),
            },
            Scope { label: "t".into() },
            "",
            "t",
        );
    }
}
