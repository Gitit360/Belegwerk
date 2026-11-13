//! List ImportBatches and document metadata for Eingang UX.

use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImportQueryError {
    #[error("{0}")]
    Storage(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportBatchRow {
    pub id: String,
    pub source_label: String,
    pub imported_at: String,
    pub note: Option<String>,
    pub document_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRow {
    pub id: String,
    pub content_hash: String,
    pub mime_type: String,
    pub original_filename: Option<String>,
    pub byte_size: i64,
    pub import_batch_id: String,
    pub imported_at: String,
    pub is_placeholder: bool,
}

pub struct ImportQuery {
    pub paths: DataPaths,
}

impl ImportQuery {
    fn conn(&self) -> Result<rusqlite::Connection, ImportQueryError> {
        self.paths
            .ensure()
            .map_err(|e| ImportQueryError::Storage(e.to_string()))?;
        let conn =
            open_db(&self.paths.db_path()).map_err(|e| ImportQueryError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| ImportQueryError::Storage(e.to_string()))?;
        Ok(conn)
    }

    pub fn list_batches(&self) -> Result<Vec<ImportBatchRow>, ImportQueryError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                "SELECT b.id, b.source_label, b.imported_at, b.note,
                        (SELECT COUNT(*) FROM documents d WHERE d.import_batch_id = b.id)
                 FROM import_batches b ORDER BY b.imported_at DESC",
            )
            .map_err(|e| ImportQueryError::Storage(e.to_string()))?;
        let rows = stmt
            .query_map([], |r| {
                Ok(ImportBatchRow {
                    id: r.get(0)?,
                    source_label: r.get(1)?,
                    imported_at: r.get(2)?,
                    note: r.get(3)?,
                    document_count: r.get(4)?,
                })
            })
            .map_err(|e| ImportQueryError::Storage(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| ImportQueryError::Storage(e.to_string()))?);
        }
        Ok(out)
    }

    pub fn list_documents(
        &self,
        batch_id: Option<&str>,
    ) -> Result<Vec<DocumentRow>, ImportQueryError> {
        let conn = self.conn()?;
        let mut out = Vec::new();
        if let Some(bid) = batch_id {
            let mut stmt = conn
                .prepare(
                    "SELECT id, content_hash, mime_type, original_filename, byte_size, import_batch_id, imported_at, is_placeholder
                     FROM documents WHERE import_batch_id = ?1 ORDER BY imported_at",
                )
                .map_err(|e| ImportQueryError::Storage(e.to_string()))?;
            let rows = stmt
                .query_map([bid], map_doc)
                .map_err(|e| ImportQueryError::Storage(e.to_string()))?;
            for row in rows {
                out.push(row.map_err(|e| ImportQueryError::Storage(e.to_string()))?);
            }
        } else {
            let mut stmt = conn
                .prepare(
                    "SELECT id, content_hash, mime_type, original_filename, byte_size, import_batch_id, imported_at, is_placeholder
                     FROM documents ORDER BY imported_at DESC LIMIT 100",
                )
                .map_err(|e| ImportQueryError::Storage(e.to_string()))?;
            let rows = stmt
                .query_map([], map_doc)
                .map_err(|e| ImportQueryError::Storage(e.to_string()))?;
            for row in rows {
                out.push(row.map_err(|e| ImportQueryError::Storage(e.to_string()))?);
            }
        }
        Ok(out)
    }
}

fn map_doc(r: &rusqlite::Row<'_>) -> Result<DocumentRow, rusqlite::Error> {
    Ok(DocumentRow {
        id: r.get(0)?,
        content_hash: r.get(1)?,
        mime_type: r.get(2)?,
        original_filename: r.get(3)?,
        byte_size: r.get(4)?,
        import_batch_id: r.get(5)?,
        imported_at: r.get(6)?,
        is_placeholder: r.get::<_, i64>(7)? != 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import_pipeline::{ImportFile, ImportPipeline};
    use crate::permission::PermissionGate;
    use tempfile::tempdir;

    #[test]
    fn lists_batches_after_import() {
        let dir = tempdir().unwrap();
        let paths = DataPaths::new(dir.path());
        let gate = PermissionGate::default();
        let pipeline = ImportPipeline {
            gate: &gate,
            paths: paths.clone(),
            caller: "test".into(),
        };
        pipeline
            .import_files(
                "dropzone",
                &[ImportFile {
                    filename: "a.txt".into(),
                    mime_type: "text/plain".into(),
                    bytes: b"hello-synth".to_vec(),
                }],
            )
            .unwrap();
        let q = ImportQuery { paths };
        let batches = q.list_batches().unwrap();
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].document_count, 1);
        assert_eq!(q.list_documents(None).unwrap().len(), 1);
    }
}
