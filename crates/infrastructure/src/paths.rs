use std::path::{Path, PathBuf};

/// Layout under a BusinessProfile data root (ADR-0002).
#[derive(Debug, Clone)]
pub struct DataPaths {
    pub root: PathBuf,
}

impl DataPaths {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn db_path(&self) -> PathBuf {
        self.root.join("belegwerk.sqlite")
    }

    pub fn objects_dir(&self) -> PathBuf {
        self.root.join("objects")
    }

    pub fn derived_dir(&self) -> PathBuf {
        self.root.join("derived")
    }

    pub fn ensure(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.root)?;
        std::fs::create_dir_all(self.objects_dir())?;
        std::fs::create_dir_all(self.derived_dir())?;
        Ok(())
    }

    pub fn object_path_for_hash(&self, hash_hex: &str) -> PathBuf {
        let (a, b) = if hash_hex.len() >= 4 {
            (&hash_hex[0..2], &hash_hex[2..4])
        } else {
            ("00", "00")
        };
        self.objects_dir().join(a).join(b).join(hash_hex)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}
