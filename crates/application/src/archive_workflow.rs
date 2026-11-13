//! Archive list + FTS search + blob text preview (M07).

use crate::extraction_service::ExtractionService;
use belegwerk_domain::ContentHash;
use belegwerk_infrastructure::{migrate, open_db, BlobStore, DataPaths, FsBlobStore};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("{0}")]
    Storage(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveDoc {
    pub id: String,
    pub content_hash: String,
    pub filename: Option<String>,
    pub mime_type: String,
    pub byte_size: i64,
    pub imported_at: String,
    pub batch_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveSnapshot {
    pub note: String,
    pub documents: Vec<ArchiveDoc>,
    pub hits: Vec<String>,
}

pub struct ArchiveWorkflow {
    pub paths: DataPaths,
}

impl ArchiveWorkflow {
    fn conn(&self) -> Result<Connection, ArchiveError> {
        self.paths
            .ensure()
            .map_err(|e| ArchiveError::Storage(e.to_string()))?;
        let conn =
            open_db(&self.paths.db_path()).map_err(|e| ArchiveError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| ArchiveError::Storage(e.to_string()))?;
        Ok(conn)
    }

    pub fn snapshot(&self, query: Option<&str>) -> Result<ArchiveSnapshot, ArchiveError> {
        let conn = self.conn()?;
        let documents = list_docs(&conn)?;
        let hits = if let Some(q) = query.filter(|s| !s.trim().is_empty()) {
            let ext = ExtractionService {
                paths: self.paths.clone(),
            };
            ext.search(q)
                .map_err(|e| ArchiveError::Storage(e.to_string()))?
        } else {
            Vec::new()
        };
        Ok(ArchiveSnapshot {
            note: "Archiv aus SQLite + content-addressed blobs".into(),
            documents,
            hits,
        })
    }

    pub fn preview_text(&self, content_hash: &str) -> Result<String, ArchiveError> {
        let store = FsBlobStore::new(self.paths.clone())
            .map_err(|e| ArchiveError::Storage(e.to_string()))?;
        let path = store.path_for(&ContentHash(content_hash.into()));
        let bytes = fs::read(&path).map_err(|e| ArchiveError::Storage(e.to_string()))?;
        String::from_utf8(bytes).map_err(|e| ArchiveError::Storage(e.to_string()))
    }
}

fn list_docs(conn: &Connection) -> Result<Vec<ArchiveDoc>, ArchiveError> {
    let mut stmt = conn
        .prepare(
            "SELECT id, content_hash, original_filename, mime_type, byte_size, imported_at, import_batch_id
             FROM documents ORDER BY imported_at DESC",
        )
        .map_err(|e| ArchiveError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([], |r| {
            Ok(ArchiveDoc {
                id: r.get(0)?,
                content_hash: r.get(1)?,
                filename: r.get(2)?,
                mime_type: r.get(3)?,
                byte_size: r.get(4)?,
                imported_at: r.get(5)?,
                batch_id: r.get(6)?,
            })
        })
        .map_err(|e| ArchiveError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(|e| ArchiveError::Storage(e.to_string()))?);
    }
    Ok(out)
}
