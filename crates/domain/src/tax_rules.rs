//! Versioned tax rule pack loader — rates/thresholds only from the cited JSON pack.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub const EMBEDDED_TAX_PROFILE_2026: &str = include_str!("../rules/tax_profile_2026.json");

#[derive(Debug, Error)]
pub enum TaxRuleError {
    #[error("parse: {0}")]
    Parse(String),
    #[error("unknown rule: {0}")]
    UnknownRule(String),
    #[error("rule {0} is blocked — no automatic posting")]
    Blocked(String),
    #[error("rule {0} requires user confirmation (encode={1})")]
    NeedsConfirm(String, String),
    #[error("private share percent must be 0..=100, got {0}")]
    BadPrivateShare(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuleEncode {
    DefaultAfterConfirm,
    SuggestOnly,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaxRuleMeta {
    pub id: String,
    pub encode: RuleEncode,
    pub confidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TaxRateEntry {
    pub rule_id: String,
    pub percent: u8,
    pub encode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TaxThresholds {
    pub voranmeldung_monthly_above_minor: i64,
    pub voranmeldung_exempt_at_or_below_minor: i64,
    pub gwg_immediate_max_net_minor: i64,
    pub sammelposten_min_net_minor: i64,
    pub sammelposten_max_net_minor: i64,
    pub kleinunternehmer_prior_year_max_minor: i64,
    pub kleinunternehmer_current_year_max_minor: i64,
    pub ist_umsatz_max_prior_year_minor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TaxPackDefaults {
    pub vat_method: String,
    pub reporting_cadence: String,
    pub dauerfrist_extension: bool,
    pub require_user_confirm_ist: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TaxRulePack {
    pub id: String,
    pub version: String,
    pub status: String,
    pub valid_from: String,
    pub valid_to: Option<String>,
    pub source_doc: String,
    pub rates: HashMap<String, TaxRateEntry>,
    pub thresholds: TaxThresholds,
    pub defaults: TaxPackDefaults,
    pub rules: Vec<TaxRuleMeta>,
}

/// Split gross amount by private-use percent (TAX-PRIVATE-SHARE).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateShareSplit {
    pub private_percent: u8,
    pub business_minor: i64,
    pub private_minor: i64,
}

impl TaxRulePack {
    pub fn load_embedded_2026() -> Result<Self, TaxRuleError> {
        Self::from_json(EMBEDDED_TAX_PROFILE_2026)
    }

    pub fn from_json(json: &str) -> Result<Self, TaxRuleError> {
        serde_json::from_str(json).map_err(|e| TaxRuleError::Parse(e.to_string()))
    }

    pub fn rule(&self, id: &str) -> Result<&TaxRuleMeta, TaxRuleError> {
        self.rules
            .iter()
            .find(|r| r.id == id)
            .ok_or_else(|| TaxRuleError::UnknownRule(id.into()))
    }

    pub fn assert_postable_rule(&self, id: &str) -> Result<&TaxRuleMeta, TaxRuleError> {
        let meta = self.rule(id)?;
        match meta.encode {
            RuleEncode::Blocked => Err(TaxRuleError::Blocked(id.into())),
            RuleEncode::SuggestOnly => {
                Err(TaxRuleError::NeedsConfirm(id.into(), "suggest_only".into()))
            }
            RuleEncode::DefaultAfterConfirm => Ok(meta),
        }
    }

    pub fn is_reviewed_or_active(&self) -> bool {
        matches!(self.status.as_str(), "reviewed" | "active")
    }

    pub fn as_ref_for_journal(&self) -> crate::accounting::TaxRulePackRef {
        crate::accounting::TaxRulePackRef {
            id: self.id.clone(),
            version: self.version.clone(),
            status: self.status.clone(),
            source_note: Some(self.source_doc.clone()),
        }
    }

    /// Split absolute amount_minor into business vs private by percent (0 = fully business).
    pub fn split_private_share(
        amount_minor: i64,
        private_percent: u8,
    ) -> Result<PrivateShareSplit, TaxRuleError> {
        if private_percent > 100 {
            return Err(TaxRuleError::BadPrivateShare(private_percent));
        }
        let abs = amount_minor.abs();
        let private_minor = (abs * i64::from(private_percent)) / 100;
        let business_minor = abs - private_minor;
        Ok(PrivateShareSplit {
            private_percent,
            business_minor,
            private_minor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_embedded_pack() {
        let pack = TaxRulePack::load_embedded_2026().expect("pack");
        assert_eq!(pack.id, "belegwerk-core");
        assert_eq!(pack.status, "reviewed");
        assert!(pack.rule("TAX-PRIVATE-SHARE").is_ok());
        assert!(pack.assert_postable_rule("TAX-PRIVATE-SHARE").is_ok());
    }

    #[test]
    fn private_share_split_30_percent() {
        let s = TaxRulePack::split_private_share(1000, 30).unwrap();
        assert_eq!(s.private_minor, 300);
        assert_eq!(s.business_minor, 700);
    }

    #[test]
    fn blocks_eu_b2b() {
        let pack = TaxRulePack::load_embedded_2026().unwrap();
        assert!(matches!(
            pack.assert_postable_rule("TAX-BLOCK-EU-B2B"),
            Err(TaxRuleError::Blocked(_))
        ));
    }
}
