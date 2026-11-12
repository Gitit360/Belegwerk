//! Persistent job stubs (reminder / import). Crash-recoverable via SQLite jobs table.

use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use chrono::Utc;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum JobError {
    #[error("{0}")]
    Storage(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobKind {
    ReminderEscalate,
    ReminderScan,
    ImportBatch,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRecord {
    pub id: String,
    pub kind: JobKind,
    pub status: JobStatus,
    pub payload_json: String,
}

pub struct JobQueue {
    pub paths: DataPaths,
}

impl JobQueue {
    fn conn(&self) -> Result<Connection, JobError> {
        self.paths
            .ensure()
            .map_err(|e| JobError::Storage(e.to_string()))?;
        let conn = open_db(&self.paths.db_path()).map_err(|e| JobError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| JobError::Storage(e.to_string()))?;
        Ok(conn)
    }

    pub fn enqueue(
        &self,
        kind: JobKind,
        payload_json: impl Into<String>,
    ) -> Result<JobRecord, JobError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let payload_json = payload_json.into();
        let kind_s = serde_json::to_string(&kind)
            .unwrap()
            .trim_matches('"')
            .to_string();
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO jobs(id, kind, status, payload_json, created_at, updated_at)
             VALUES (?1, ?2, 'queued', ?3, ?4, ?4)",
            (&id, &kind_s, &payload_json, &now),
        )
        .map_err(|e| JobError::Storage(e.to_string()))?;
        Ok(JobRecord {
            id,
            kind,
            status: JobStatus::Queued,
            payload_json,
        })
    }

    pub fn mark(&self, id: &str, status: JobStatus) -> Result<(), JobError> {
        let status_s = serde_json::to_string(&status)
            .unwrap()
            .trim_matches('"')
            .to_string();
        let conn = self.conn()?;
        conn.execute(
            "UPDATE jobs SET status = ?1, updated_at = ?2 WHERE id = ?3",
            (status_s, Utc::now().to_rfc3339(), id),
        )
        .map_err(|e| JobError::Storage(e.to_string()))?;
        Ok(())
    }

    pub fn list_queued(&self) -> Result<Vec<(String, String)>, JobError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare("SELECT id, kind FROM jobs WHERE status = 'queued' ORDER BY created_at")
            .map_err(|e| JobError::Storage(e.to_string()))?;
        let rows = stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
            .map_err(|e| JobError::Storage(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| JobError::Storage(e.to_string()))?);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn enqueue_reminder_job() {
        let dir = tempdir().unwrap();
        let q = JobQueue {
            paths: DataPaths::new(dir.path()),
        };
        let job = q
            .enqueue(JobKind::ReminderEscalate, r#"{"caseId":"x"}"#)
            .unwrap();
        assert_eq!(job.status, JobStatus::Queued);
        assert_eq!(q.list_queued().unwrap().len(), 1);
        q.mark(&job.id, JobStatus::Succeeded).unwrap();
        assert!(q.list_queued().unwrap().is_empty());
    }
}
