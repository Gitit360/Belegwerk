//! Document archive application stubs (M03).

use belegwerk_domain::time::UtcInstant;
use belegwerk_domain::{ContentHash, DocumentId, DocumentMeta, ImportBatch, ImportBatchId};
use chrono::Utc;

pub struct ImportedDocument {
    pub batch: ImportBatch,
    pub meta: DocumentMeta,
}

pub fn build_document_meta(
    hash: ContentHash,
    mime_type: impl Into<String>,
    byte_size: u64,
    batch_id: ImportBatchId,
    original_filename: Option<String>,
) -> DocumentMeta {
    DocumentMeta {
        id: DocumentId::new(),
        content_hash: hash,
        mime_type: mime_type.into(),
        original_filename,
        byte_size,
        import_batch_id: batch_id,
        imported_at: Utc::now(),
        document_date: None,
        is_placeholder: false,
    }
}

pub fn new_batch(source_label: impl Into<String>) -> ImportBatch {
    ImportBatch::new(source_label)
}

pub fn now() -> UtcInstant {
    Utc::now()
}
