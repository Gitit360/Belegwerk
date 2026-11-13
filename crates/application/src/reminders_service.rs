//! Missing-document cases and placeholders (M06).

use belegwerk_domain::ids::EconomicTransactionId;
use belegwerk_domain::reminders::{DocumentPlaceholder, MissingDocumentCase};
use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use rusqlite::Connection;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReminderError {
    #[error("{0}")]
    Storage(String),
}

pub struct ReminderService {
    pub paths: DataPaths,
}

impl ReminderService {
    fn conn(&self) -> Result<Connection, ReminderError> {
        self.paths
            .ensure()
            .map_err(|e| ReminderError::Storage(e.to_string()))?;
        let conn =
            open_db(&self.paths.db_path()).map_err(|e| ReminderError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| ReminderError::Storage(e.to_string()))?;
        Ok(conn)
    }

    pub fn open_case(
        &self,
        note: impl Into<String>,
        economic_transaction_id: Option<EconomicTransactionId>,
    ) -> Result<MissingDocumentCase, ReminderError> {
        let mut case = MissingDocumentCase::open(note);
        case.economic_transaction_id = economic_transaction_id;
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO missing_document_cases(id, economic_transaction_id, status, escalation_level, note, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                case.id.to_string(),
                case.economic_transaction_id.map(|id| id.to_string()),
                "open",
                case.escalation_level as i64,
                &case.note,
                case.created_at.to_rfc3339(),
            ),
        )
        .map_err(|e| ReminderError::Storage(e.to_string()))?;
        Ok(case)
    }

    pub fn attach_placeholder(
        &self,
        case_id: uuid::Uuid,
        label: impl Into<String>,
    ) -> Result<DocumentPlaceholder, ReminderError> {
        let mut ph = DocumentPlaceholder::new(label);
        ph.case_id = Some(case_id);
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO document_placeholders(id, case_id, label, created_at) VALUES (?1, ?2, ?3, ?4)",
            (
                ph.id.to_string(),
                ph.case_id.map(|id| id.to_string()),
                &ph.label,
                ph.created_at.to_rfc3339(),
            ),
        )
        .map_err(|e| ReminderError::Storage(e.to_string()))?;
        Ok(ph)
    }

    pub fn escalate(&self, case_id: uuid::Uuid) -> Result<u8, ReminderError> {
        let conn = self.conn()?;
        conn.execute(
            "UPDATE missing_document_cases
             SET escalation_level = escalation_level + 1,
                 status = CASE WHEN escalation_level + 1 >= 2 THEN 'waiting_on_mail' ELSE status END
             WHERE id = ?1",
            [case_id.to_string()],
        )
        .map_err(|e| ReminderError::Storage(e.to_string()))?;
        let level: i64 = conn
            .query_row(
                "SELECT escalation_level FROM missing_document_cases WHERE id = ?1",
                [case_id.to_string()],
                |r| r.get(0),
            )
            .map_err(|e| ReminderError::Storage(e.to_string()))?;
        Ok(level as u8)
    }

    pub fn list_open(&self) -> Result<Vec<(String, String, i64)>, ReminderError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                "SELECT id, COALESCE(note, ''), escalation_level FROM missing_document_cases
                 WHERE status != 'resolved' AND status != 'dismissed'
                 ORDER BY created_at",
            )
            .map_err(|e| ReminderError::Storage(e.to_string()))?;
        let rows = stmt
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, i64>(2)?,
                ))
            })
            .map_err(|e| ReminderError::Storage(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| ReminderError::Storage(e.to_string()))?);
        }
        Ok(out)
    }

    pub fn list_open_detailed(&self) -> Result<Vec<MissingCaseRow>, ReminderError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                "SELECT id, COALESCE(note, ''), escalation_level, status, created_at,
                        COALESCE(economic_transaction_id, '')
                 FROM missing_document_cases
                 WHERE status != 'resolved' AND status != 'dismissed'
                 ORDER BY created_at",
            )
            .map_err(|e| ReminderError::Storage(e.to_string()))?;
        let rows = stmt
            .query_map([], |r| {
                Ok(MissingCaseRow {
                    id: r.get(0)?,
                    note: r.get(1)?,
                    escalation_level: r.get(2)?,
                    status: r.get(3)?,
                    created_at: r.get(4)?,
                    economic_transaction_id: {
                        let s: String = r.get(5)?;
                        if s.is_empty() {
                            None
                        } else {
                            Some(s)
                        }
                    },
                })
            })
            .map_err(|e| ReminderError::Storage(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| ReminderError::Storage(e.to_string()))?);
        }
        Ok(out)
    }

    pub fn complete(&self, case_id: uuid::Uuid) -> Result<(), ReminderError> {
        let conn = self.conn()?;
        let n = conn
            .execute(
                "UPDATE missing_document_cases SET status = 'resolved' WHERE id = ?1",
                [case_id.to_string()],
            )
            .map_err(|e| ReminderError::Storage(e.to_string()))?;
        if n == 0 {
            return Err(ReminderError::Storage(format!("case not found: {case_id}")));
        }
        Ok(())
    }

    pub fn seed_demo_if_empty(&self) -> Result<(), ReminderError> {
        let conn = self.conn()?;
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM missing_document_cases", [], |r| {
                r.get(0)
            })
            .map_err(|e| ReminderError::Storage(e.to_string()))?;
        if n > 0 {
            return Ok(());
        }
        drop(conn);
        let case = self.open_case("Beleg zu mm-synth-001 fehlt (synthetisch)", None)?;
        self.attach_placeholder(case.id, "DocumentPlaceholder: Vendor PDF ausstehend")?;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissingCaseRow {
    pub id: String,
    pub note: String,
    pub escalation_level: i64,
    pub status: String,
    pub created_at: String,
    pub economic_transaction_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use belegwerk_domain::reminders::MissingStatus;
    use tempfile::tempdir;

    #[test]
    fn opens_case_with_placeholder_and_escalation() {
        let dir = tempdir().unwrap();
        let svc = ReminderService {
            paths: DataPaths::new(dir.path()),
        };
        let case = svc.open_case("missing invoice for synth tx", None).unwrap();
        assert_eq!(case.status, MissingStatus::Open);
        let ph = svc
            .attach_placeholder(case.id, "DocumentPlaceholder: awaiting vendor PDF")
            .unwrap();
        assert!(ph.case_id.is_some());
        let level = svc.escalate(case.id).unwrap();
        assert_eq!(level, 1);
        assert_eq!(svc.list_open().unwrap().len(), 1);
        svc.complete(case.id).unwrap();
        assert!(svc.list_open().unwrap().is_empty());
    }
}
