//! Document archive types — originals are content-addressed (D-11).

use crate::ids::{DocumentId, ImportBatchId};
use crate::time::{CalendarDate, UtcInstant};
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// SHA-256 hex digest of original bytes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash(pub String);

impl ContentHash {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Path key under `<data_root>/objects/ab/cd/<sha256>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectKey {
    pub hash: ContentHash,
}

impl ObjectKey {
    pub fn from_hash(hash: ContentHash) -> Self {
        Self { hash }
    }

    /// Relative path segments: `objects/{aa}/{bb}/{fullhash}`.
    pub fn relative_path(&self) -> String {
        let h = self.hash.as_str();
        let (a, b) = if h.len() >= 4 {
            (&h[0..2], &h[2..4])
        } else {
            ("00", "00")
        };
        format!("objects/{a}/{b}/{h}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportBatch {
    pub id: ImportBatchId,
    pub source_label: String,
    pub imported_at: UtcInstant,
    pub note: Option<String>,
}

impl ImportBatch {
    pub fn new(source_label: impl Into<String>) -> Self {
        Self {
            id: ImportBatchId::new(),
            source_label: source_label.into(),
            imported_at: Utc::now(),
            note: None,
        }
    }
}

/// Metadata only — bytes live in the object store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMeta {
    pub id: DocumentId,
    pub content_hash: ContentHash,
    pub mime_type: String,
    pub original_filename: Option<String>,
    pub byte_size: u64,
    pub import_batch_id: ImportBatchId,
    pub imported_at: UtcInstant,
    /// Optional document date (calendar, Europe/Berlin interpretation).
    pub document_date: Option<CalendarDate>,
    /// True only for DocumentPlaceholder / proxy — never grants Vorsteuer alone.
    pub is_placeholder: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_key_path_uses_hash_prefix() {
        let key = ObjectKey::from_hash(ContentHash(
            "abcd1234eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".into(),
        ));
        assert_eq!(
            key.relative_path(),
            "objects/ab/cd/abcd1234eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
        );
    }
}
