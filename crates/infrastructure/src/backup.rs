//! Backup helpers (M14 stub) — no claim of completed restore drills.

use crate::paths::DataPaths;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct BackupPlan {
    pub db: PathBuf,
    pub objects: PathBuf,
    pub derived: PathBuf,
}

impl BackupPlan {
    pub fn from_paths(paths: &DataPaths) -> Self {
        Self {
            db: paths.db_path(),
            objects: paths.objects_dir(),
            derived: paths.derived_dir(),
        }
    }

    /// Copies DB file if present; objects/derived directories best-effort.
    /// Does not encrypt; encryption pin still open (ADR-0002).
    pub fn copy_to(&self, dest_root: &Path) -> io::Result<Vec<PathBuf>> {
        fs::create_dir_all(dest_root)?;
        let mut copied = Vec::new();
        if self.db.exists() {
            let dest = dest_root.join("belegwerk.sqlite");
            fs::copy(&self.db, &dest)?;
            copied.push(dest);
        }
        for (src, name) in [(&self.objects, "objects"), (&self.derived, "derived")] {
            if src.exists() {
                let dest = dest_root.join(name);
                copy_dir(src, &dest)?;
                copied.push(dest);
            }
        }
        Ok(copied)
    }
}

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dest.join(entry.file_name());
        if ty.is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{migrate, open_db, DataPaths};
    use tempfile::tempdir;

    #[test]
    fn backup_copies_db() {
        let dir = tempdir().unwrap();
        let paths = DataPaths::new(dir.path().join("data"));
        paths.ensure().unwrap();
        let conn = open_db(&paths.db_path()).unwrap();
        migrate(&conn).unwrap();
        let backup_root = dir.path().join("backup");
        let plan = BackupPlan::from_paths(&paths);
        let copied = plan.copy_to(&backup_root).unwrap();
        assert!(copied.iter().any(|p| p.ends_with("belegwerk.sqlite")));
    }
}
