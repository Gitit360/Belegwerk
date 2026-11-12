use crate::paths::DataPaths;
use belegwerk_domain::{ContentHash, ObjectKey};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlobStoreError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("not found: {0}")]
    NotFound(String),
}

pub trait BlobStore {
    fn put_bytes(&self, bytes: &[u8]) -> Result<ObjectKey, BlobStoreError>;
    fn exists(&self, hash: &ContentHash) -> bool;
    fn path_for(&self, hash: &ContentHash) -> PathBuf;
}

#[derive(Debug, Clone)]
pub struct FsBlobStore {
    paths: DataPaths,
}

impl FsBlobStore {
    pub fn new(paths: DataPaths) -> Result<Self, BlobStoreError> {
        paths.ensure()?;
        Ok(Self { paths })
    }

    pub fn hash_bytes(bytes: &[u8]) -> ContentHash {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        ContentHash(hex::encode(hasher.finalize()))
    }
}

impl BlobStore for FsBlobStore {
    fn put_bytes(&self, bytes: &[u8]) -> Result<ObjectKey, BlobStoreError> {
        let hash = Self::hash_bytes(bytes);
        let dest = self.paths.object_path_for_hash(hash.as_str());
        if dest.exists() {
            return Ok(ObjectKey::from_hash(hash));
        }
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let tmp = dest.with_extension("tmp");
        {
            let mut f = fs::File::create(&tmp)?;
            f.write_all(bytes)?;
            f.sync_all()?;
        }
        fs::rename(&tmp, &dest)?;
        Ok(ObjectKey::from_hash(hash))
    }

    fn exists(&self, hash: &ContentHash) -> bool {
        self.paths.object_path_for_hash(hash.as_str()).exists()
    }

    fn path_for(&self, hash: &ContentHash) -> PathBuf {
        self.paths.object_path_for_hash(hash.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn content_addressed_put_is_idempotent() {
        let dir = tempdir().unwrap();
        let store = FsBlobStore::new(DataPaths::new(dir.path())).unwrap();
        let a = store.put_bytes(b"hello-belegwerk").unwrap();
        let b = store.put_bytes(b"hello-belegwerk").unwrap();
        assert_eq!(a.hash, b.hash);
        assert!(store.exists(&a.hash));
    }
}
