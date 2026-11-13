//! TaxProfile local settings in SQLite — no ELSTER (D-05, D-19).

use crate::actions::KnownAction;
use crate::permission::{PermissionGate, PermissionRequest, ResourceRef, Scope};
use belegwerk_domain::settings::TaxProfileSettings;
use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use chrono::Utc;
use thiserror::Error;

const KEY: &str = "tax_profile";

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("permission denied: {0}")]
    Denied(String),
    #[error("{0}")]
    Storage(String),
}

pub struct SettingsService<'a> {
    pub gate: &'a PermissionGate,
    pub paths: DataPaths,
    pub caller: String,
}

impl SettingsService<'_> {
    pub fn get_tax_profile(&self) -> Result<TaxProfileSettings, SettingsError> {
        self.paths
            .ensure()
            .map_err(|e| SettingsError::Storage(e.to_string()))?;
        let conn =
            open_db(&self.paths.db_path()).map_err(|e| SettingsError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| SettingsError::Storage(e.to_string()))?;
        let json: Option<String> = conn
            .query_row(
                "SELECT value_json FROM settings_kv WHERE key = ?1",
                [KEY],
                |r| r.get(0),
            )
            .ok();
        match json {
            Some(j) => serde_json::from_str(&j).map_err(|e| SettingsError::Storage(e.to_string())),
            None => {
                let defaults = TaxProfileSettings::default();
                self.write_raw(&defaults)?;
                Ok(defaults)
            }
        }
    }

    pub fn save_tax_profile(
        &self,
        profile: &TaxProfileSettings,
    ) -> Result<TaxProfileSettings, SettingsError> {
        let req = PermissionRequest::for_known(
            KnownAction::SettingsWrite,
            ResourceRef {
                kind: "settings".into(),
                id: Some(KEY.into()),
            },
            Scope {
                label: "settings".into(),
            },
            &serde_json::to_string(profile).unwrap_or_default(),
            &self.caller,
        );
        self.gate
            .authorize(&req)
            .map_err(|e| SettingsError::Denied(e.to_string()))?;
        self.write_raw(profile)?;
        Ok(profile.clone())
    }

    fn write_raw(&self, profile: &TaxProfileSettings) -> Result<(), SettingsError> {
        self.paths
            .ensure()
            .map_err(|e| SettingsError::Storage(e.to_string()))?;
        let conn =
            open_db(&self.paths.db_path()).map_err(|e| SettingsError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| SettingsError::Storage(e.to_string()))?;
        let json =
            serde_json::to_string(profile).map_err(|e| SettingsError::Storage(e.to_string()))?;
        conn.execute(
            "INSERT INTO settings_kv(key, value_json, updated_at) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value_json=excluded.value_json, updated_at=excluded.updated_at",
            (KEY, json, Utc::now().to_rfc3339()),
        )
        .map_err(|e| SettingsError::Storage(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use belegwerk_domain::settings::{ReportingCadence, VatMethod};
    use tempfile::tempdir;

    #[test]
    fn defaults_then_persist_dauerfrist_off() {
        let dir = tempdir().unwrap();
        let gate = PermissionGate::default();
        let svc = SettingsService {
            gate: &gate,
            paths: DataPaths::new(dir.path()),
            caller: "test".into(),
        };
        let p = svc.get_tax_profile().unwrap();
        assert_eq!(p.vat_method, VatMethod::Ist);
        assert_eq!(p.reporting_cadence, ReportingCadence::Monthly);
        assert!(!p.dauerfrist_extension);
        let mut next = p.clone();
        next.tax_year = 2027;
        svc.save_tax_profile(&next).unwrap();
        assert_eq!(svc.get_tax_profile().unwrap().tax_year, 2027);
        assert!(!svc.get_tax_profile().unwrap().dauerfrist_extension);
    }
}
