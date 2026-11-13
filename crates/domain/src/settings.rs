//! Local settings stubs — TaxProfile defaults (D-05, D-19). No ELSTER.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VatMethod {
    /// Istversteuerung (default for v1 freelancers).
    Ist,
    Soll,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReportingCadence {
    Monthly,
    Quarterly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaxProfileSettings {
    pub vat_method: VatMethod,
    pub reporting_cadence: ReportingCadence,
    /// Dauerfristverlängerung — default off (D-05).
    pub dauerfrist_extension: bool,
    /// Suggested current calendar year; user confirms (D-19).
    pub tax_year: i32,
}

impl Default for TaxProfileSettings {
    fn default() -> Self {
        Self {
            vat_method: VatMethod::Ist,
            reporting_cadence: ReportingCadence::Monthly,
            dauerfrist_extension: false,
            tax_year: 2026,
        }
    }
}
