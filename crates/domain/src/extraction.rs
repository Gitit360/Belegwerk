//! OCR / extraction stubs — provenance required; no invented fields (REQ-O-002).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedField {
    pub name: String,
    pub value: Option<String>,
    pub confidence: f32,
    pub provenance: FieldProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldProvenance {
    Ocr,
    Parser,
    User,
    AiSuggestion,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    pub document_hash: String,
    pub fields: Vec<ExtractedField>,
}

impl ExtractionResult {
    /// Stub: marks all requested fields missing rather than inventing values.
    pub fn empty_stub(document_hash: impl Into<String>, field_names: &[&str]) -> Self {
        Self {
            document_hash: document_hash.into(),
            fields: field_names
                .iter()
                .map(|n| ExtractedField {
                    name: (*n).into(),
                    value: None,
                    confidence: 0.0,
                    provenance: FieldProvenance::Missing,
                })
                .collect(),
        }
    }
}
