//! Text extraction stub + FTS rebuild hooks (M07). Never invents missing fields.

use belegwerk_domain::extraction::{ExtractedField, ExtractionResult, FieldProvenance};
use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use rusqlite::Connection;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtractionError {
    #[error("{0}")]
    Storage(String),
}

pub struct ExtractionService {
    pub paths: DataPaths,
}

impl ExtractionService {
    fn conn(&self) -> Result<Connection, ExtractionError> {
        self.paths
            .ensure()
            .map_err(|e| ExtractionError::Storage(e.to_string()))?;
        let conn =
            open_db(&self.paths.db_path()).map_err(|e| ExtractionError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| ExtractionError::Storage(e.to_string()))?;
        ensure_fts(&conn).map_err(|e| ExtractionError::Storage(e.to_string()))?;
        Ok(conn)
    }

    /// Parse only clearly labeled KEY: value lines; unknowns stay Missing.
    pub fn extract_labeled_text(
        &self,
        document_hash: &str,
        text: &str,
        wanted: &[&str],
    ) -> ExtractionResult {
        let mut fields = Vec::new();
        for name in wanted {
            let mut found = None;
            for line in text.lines() {
                let line = line.trim();
                if let Some((k, v)) = line.split_once(':') {
                    if k.trim().eq_ignore_ascii_case(name) {
                        let val = v.trim();
                        if !val.is_empty() {
                            found = Some(val.to_string());
                        }
                    }
                }
            }
            fields.push(ExtractedField {
                name: (*name).into(),
                value: found.clone(),
                confidence: if found.is_some() { 0.8 } else { 0.0 },
                provenance: if found.is_some() {
                    FieldProvenance::Parser
                } else {
                    FieldProvenance::Missing
                },
            });
        }
        ExtractionResult {
            document_hash: document_hash.into(),
            fields,
        }
    }

    pub fn index_document_text(
        &self,
        document_hash: &str,
        text: &str,
    ) -> Result<(), ExtractionError> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO document_fts(document_hash, body) VALUES (?1, ?2)
             ON CONFLICT(document_hash) DO UPDATE SET body = excluded.body",
            (document_hash, text),
        )
        .map_err(|e| ExtractionError::Storage(e.to_string()))?;
        Ok(())
    }

    pub fn search(&self, query: &str) -> Result<Vec<String>, ExtractionError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                "SELECT document_hash FROM document_fts WHERE body LIKE ?1 ORDER BY document_hash",
            )
            .map_err(|e| ExtractionError::Storage(e.to_string()))?;
        let pattern = format!("%{query}%");
        let rows = stmt
            .query_map([pattern], |r| r.get::<_, String>(0))
            .map_err(|e| ExtractionError::Storage(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| ExtractionError::Storage(e.to_string()))?);
        }
        Ok(out)
    }

    pub fn rebuild_index_from_pairs(
        &self,
        pairs: &[(String, String)],
    ) -> Result<usize, ExtractionError> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM document_fts", [])
            .map_err(|e| ExtractionError::Storage(e.to_string()))?;
        for (hash, body) in pairs {
            conn.execute(
                "INSERT INTO document_fts(document_hash, body) VALUES (?1, ?2)",
                (hash, body),
            )
            .map_err(|e| ExtractionError::Storage(e.to_string()))?;
        }
        Ok(pairs.len())
    }
}

fn ensure_fts(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS document_fts (
            document_hash TEXT PRIMARY KEY NOT NULL,
            body TEXT NOT NULL
         );",
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn extracts_labeled_fields_without_invention() {
        let dir = tempdir().unwrap();
        let svc = ExtractionService {
            paths: DataPaths::new(dir.path()),
        };
        let text = "Invoice: INV-SYNTH-1001\nAmount: EUR 24.99\n";
        let res = svc.extract_labeled_text("hash1", text, &["Invoice", "Vendor", "Amount"]);
        assert_eq!(
            res.fields
                .iter()
                .find(|f| f.name == "Invoice")
                .unwrap()
                .value
                .as_deref(),
            Some("INV-SYNTH-1001")
        );
        assert!(res
            .fields
            .iter()
            .find(|f| f.name == "Vendor")
            .unwrap()
            .value
            .is_none());
        svc.index_document_text("hash1", text).unwrap();
        assert_eq!(svc.search("INV-SYNTH").unwrap(), vec!["hash1".to_string()]);
        assert_eq!(
            svc.rebuild_index_from_pairs(&[("hash1".into(), text.into())])
                .unwrap(),
            1
        );
    }
}
