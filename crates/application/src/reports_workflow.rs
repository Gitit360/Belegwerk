//! Reports aggregated from SQLite allocations + journal (M12).

use crate::reports::{ReportLine, ReportShellView};
use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use rusqlite::Connection;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReportsError {
    #[error("{0}")]
    Storage(String),
}

pub struct ReportsWorkflow {
    pub paths: DataPaths,
}

impl ReportsWorkflow {
    pub fn snapshot(&self) -> Result<ReportShellView, ReportsError> {
        self.paths
            .ensure()
            .map_err(|e| ReportsError::Storage(e.to_string()))?;
        let conn =
            open_db(&self.paths.db_path()).map_err(|e| ReportsError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| ReportsError::Storage(e.to_string()))?;

        let mut lines = Vec::new();
        lines.extend(allocation_lines(&conn)?);
        lines.extend(journal_lines(&conn)?);

        if lines.is_empty() {
            lines.push(ReportLine {
                label: "Keine gebuchten oder zugeordneten Beträge".into(),
                amount_minor: 0,
                currency: "EUR".into(),
                source_ref: None,
            });
        }

        Ok(ReportShellView {
            id: "cash-from-db".into(),
            title: "Cash / Zuordnungen (SQLite)".into(),
            basis: "management".into(),
            lines,
            note: "Aggregiert aus payment_allocations und journal_entries — kein Steuerformular."
                .into(),
        })
    }
}

fn allocation_lines(conn: &Connection) -> Result<Vec<ReportLine>, ReportsError> {
    let mut stmt = conn
        .prepare(
            "SELECT id, allocated_minor, currency, economic_transaction_id
             FROM payment_allocations ORDER BY created_at",
        )
        .map_err(|e| ReportsError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            ))
        })
        .map_err(|e| ReportsError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        let (id, amount, currency, eco) =
            row.map_err(|e| ReportsError::Storage(e.to_string()))?;
        out.push(ReportLine {
            label: format!("Zuordnung {id}"),
            amount_minor: -amount.abs(),
            currency,
            source_ref: Some(eco),
        });
    }
    Ok(out)
}

fn journal_lines(conn: &Connection) -> Result<Vec<ReportLine>, ReportsError> {
    let mut stmt = conn
        .prepare(
            "SELECT id, COALESCE(description, id), status, blocked_unsupported
             FROM journal_entries ORDER BY id",
        )
        .map_err(|e| ReportsError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, i64>(3)?,
            ))
        })
        .map_err(|e| ReportsError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        let (id, desc, status, blocked) =
            row.map_err(|e| ReportsError::Storage(e.to_string()))?;
        let sum: i64 = conn
            .query_row(
                "SELECT COALESCE(SUM(debit_minor),0) FROM journal_lines WHERE journal_entry_id = ?1",
                [&id],
                |r| r.get(0),
            )
            .unwrap_or(0);
        out.push(ReportLine {
            label: format!("{desc} [{status}]"),
            amount_minor: if blocked != 0 { 0 } else { -sum },
            currency: "EUR".into(),
            source_ref: Some(id),
        });
    }
    Ok(out)
}
