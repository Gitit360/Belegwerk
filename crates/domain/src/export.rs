use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportManifest {
    pub format: String,
    pub created_at: String,
    pub checksum_algo: String,
    pub entries: Vec<ExportEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportEntry {
    pub path: String,
    pub sha256: String,
    pub byte_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPackageStub {
    pub manifest: ExportManifest,
    pub note: String,
}

impl ExportPackageStub {
    pub fn empty_archive() -> Self {
        Self {
            manifest: ExportManifest {
                format: "belegwerk-archive-v0".into(),
                created_at: "1970-01-01T00:00:00Z".into(),
                checksum_algo: "sha256".into(),
                entries: vec![],
            },
            note: "Stub only — full export package in M13".into(),
        }
    }
}
