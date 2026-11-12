//! Import pipeline: authorize → hash → blob store → ImportBatch + Document rows.

use crate::actions::KnownAction;
use crate::archive::build_document_meta;
use crate::permission::{PermissionGate, PermissionRequest, ResourceRef, Scope};
use belegwerk_domain::{DocumentMeta, ImportBatch};
use belegwerk_infrastructure::{migrate, open_db, BlobStore, DataPaths, FsBlobStore};
use rusqlite::Connection;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("permission denied: {0}")]
    Denied(String),
    #[error("io/db: {0}")]
    Storage(String),
}

#[derive(Debug, Clone)]
pub struct ImportFile {
    pub filename: String,
    pub mime_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct ImportOutcome {
    pub batch: ImportBatch,
    pub documents: Vec<DocumentMeta>,
}

pub struct ImportPipeline<'a> {
    pub gate: &'a PermissionGate,
    pub paths: DataPaths,
    pub caller: String,
}

impl ImportPipeline<'_> {
    pub fn import_files(
        &self,
        source_label: impl Into<String>,
        files: &[ImportFile],
    ) -> Result<ImportOutcome, ImportError> {
        let source_label = source_label.into();
        let req = PermissionRequest::for_known(
            KnownAction::DocumentsImport,
            ResourceRef {
                kind: "document".into(),
                id: None,
            },
            Scope {
                label: "import".into(),
            },
            &format!("{source_label}:{}", files.len()),
            &self.caller,
        );
        self.gate
            .authorize(&req)
            .map_err(|e| ImportError::Denied(e.to_string()))?;

        self.paths
            .ensure()
            .map_err(|e| ImportError::Storage(e.to_string()))?;
        let store = FsBlobStore::new(self.paths.clone())
            .map_err(|e| ImportError::Storage(e.to_string()))?;
        let conn =
            open_db(&self.paths.db_path()).map_err(|e| ImportError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| ImportError::Storage(e.to_string()))?;

        let batch = ImportBatch::new(source_label);
        insert_batch(&conn, &batch).map_err(|e| ImportError::Storage(e.to_string()))?;

        let mut documents = Vec::new();
        for file in files {
            let key = store
                .put_bytes(&file.bytes)
                .map_err(|e| ImportError::Storage(e.to_string()))?;
            let meta = build_document_meta(
                key.hash.clone(),
                file.mime_type.clone(),
                file.bytes.len() as u64,
                batch.id,
                Some(file.filename.clone()),
            );
            insert_document(&conn, &meta).map_err(|e| ImportError::Storage(e.to_string()))?;
            documents.push(meta);
        }

        Ok(ImportOutcome { batch, documents })
    }
}

fn insert_batch(conn: &Connection, batch: &ImportBatch) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO import_batches(id, source_label, imported_at, note) VALUES (?1, ?2, ?3, ?4)",
        (
            batch.id.to_string(),
            &batch.source_label,
            batch.imported_at.to_rfc3339(),
            &batch.note,
        ),
    )?;
    Ok(())
}

fn insert_document(conn: &Connection, meta: &DocumentMeta) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO documents(id, content_hash, mime_type, original_filename, byte_size, import_batch_id, imported_at, document_date, is_placeholder)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            meta.id.to_string(),
            meta.content_hash.as_str(),
            &meta.mime_type,
            &meta.original_filename,
            meta.byte_size as i64,
            meta.import_batch_id.to_string(),
            meta.imported_at.to_rfc3339(),
            meta.document_date.map(|d| d.0.to_string()),
            meta.is_placeholder as i64,
        ),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn imports_synthetic_bytes_idempotent_blob() {
        let dir = tempdir().unwrap();
        let paths = DataPaths::new(dir.path());
        let gate = PermissionGate::default();
        let pipeline = ImportPipeline {
            gate: &gate,
            paths: paths.clone(),
            caller: "test".into(),
        };
        let bytes = b"SYNTHETIC INVOICE - NOT A REAL BELEG".to_vec();
        let out1 = pipeline
            .import_files(
                "synthetic-fixture",
                &[ImportFile {
                    filename: "synth-invoice.txt".into(),
                    mime_type: "text/plain".into(),
                    bytes: bytes.clone(),
                }],
            )
            .unwrap();
        assert_eq!(out1.documents.len(), 1);
        let out2 = pipeline
            .import_files(
                "synthetic-fixture-2",
                &[ImportFile {
                    filename: "synth-invoice-copy.txt".into(),
                    mime_type: "text/plain".into(),
                    bytes,
                }],
            )
            .unwrap();
        assert_eq!(
            out1.documents[0].content_hash,
            out2.documents[0].content_hash
        );
        let conn = open_db(&paths.db_path()).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM documents", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 2);
    }
}
