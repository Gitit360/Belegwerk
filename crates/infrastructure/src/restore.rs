//! Restore helpers + roundtrip test on synthetic DB+blobs (M14).

use crate::paths::DataPaths;
use std::fs;
use std::io;
use std::path::Path;

pub fn restore_from_backup(backup_root: &Path, dest: &DataPaths) -> io::Result<()> {
    dest.ensure()?;
    let src_db = backup_root.join("belegwerk.sqlite");
    if src_db.exists() {
        fs::copy(src_db, dest.db_path())?;
    }
    for name in ["objects", "derived"] {
        let src = backup_root.join(name);
        if src.exists() {
            let target = dest.root().join(name);
            if target.exists() {
                fs::remove_dir_all(&target)?;
            }
            copy_dir(&src, &target)?;
        }
    }
    Ok(())
}

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let to = dest.join(entry.file_name());
        if entry.file_type()?.is_dir() {
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
    use crate::backup::BackupPlan;
    use crate::blob_store::{BlobStore, FsBlobStore};
    use crate::db::{migrate, open_db};
    use tempfile::tempdir;

    #[test]
    fn backup_restore_roundtrip_preserves_blob_and_schema() {
        let dir = tempdir().unwrap();
        let data = DataPaths::new(dir.path().join("data"));
        data.ensure().unwrap();
        let conn = open_db(&data.db_path()).unwrap();
        let version = migrate(&conn).unwrap();
        let store = FsBlobStore::new(data.clone()).unwrap();
        let key = store.put_bytes(b"synthetic-beleg-bytes").unwrap();

        let backup_root = dir.path().join("backup");
        BackupPlan::from_paths(&data).copy_to(&backup_root).unwrap();

        // wipe and restore
        fs::remove_dir_all(data.root()).unwrap();
        let restored = DataPaths::new(dir.path().join("data"));
        restore_from_backup(&backup_root, &restored).unwrap();
        let conn2 = open_db(&restored.db_path()).unwrap();
        let v2 = migrate(&conn2).unwrap();
        assert_eq!(v2, version);
        let store2 = FsBlobStore::new(restored).unwrap();
        assert!(store2.exists(&key.hash));
        // integrity: schema_migrations preserved
        let conn3 = open_db(&DataPaths::new(dir.path().join("data")).db_path()).unwrap();
        let n: i64 = conn3
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |r| r.get(0))
            .unwrap();
        assert!(n >= 1);
        let bytes = std::fs::read(store2.path_for(&key.hash)).unwrap();
        assert_eq!(bytes, b"synthetic-beleg-bytes");
    }
}
