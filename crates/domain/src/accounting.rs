//! SKR03-oriented structures without inventing tax rates.
//! Unsupported cases → BookingState::BlockedUnsupported.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartAccount {
    pub code: String,
    pub name: String,
    pub kind: AccountKind,
    pub skr_variant: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountKind {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
    Tax,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRulePackRef {
    pub id: String,
    pub version: String,
    /// `draft` | `reviewed` | `active` | `retired`
    pub status: String,
    pub source_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntryDraft {
    pub id: String,
    pub description: Option<String>,
    pub lines: Vec<JournalLine>,
    pub rule_pack: Option<TaxRulePackRef>,
    /// If true, posting is refused until a reviewed rule pack exists.
    pub blocked_unsupported: bool,
    pub block_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLine {
    pub account_code: String,
    pub debit_minor: i64,
    pub credit_minor: i64,
    pub currency: String,
}

impl JournalEntryDraft {
    pub fn blocked(id: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: None,
            lines: vec![],
            rule_pack: None,
            blocked_unsupported: true,
            block_reason: Some(reason.into()),
        }
    }

    pub fn is_postable(&self) -> bool {
        !self.blocked_unsupported
            && self
                .lines
                .iter()
                .any(|l| l.debit_minor > 0 || l.credit_minor > 0)
            && self
                .rule_pack
                .as_ref()
                .map(|r| r.status == "active" || r.status == "reviewed")
                .unwrap_or(false)
    }
}

/// Minimal SKR03-oriented seed — names only, no tax computation.
pub fn skr03_seed_accounts() -> Vec<ChartAccount> {
    vec![
        ChartAccount {
            code: "1200".into(),
            name: "Bank".into(),
            kind: AccountKind::Asset,
            skr_variant: "SKR03".into(),
        },
        ChartAccount {
            code: "8400".into(),
            name: "Erlöse 19% (Kontorahmen-Platzhalter)".into(),
            kind: AccountKind::Revenue,
            skr_variant: "SKR03".into(),
        },
        ChartAccount {
            code: "4950".into(),
            name: "Sonstige betriebliche Aufwendungen (Platzhalter)".into(),
            kind: AccountKind::Expense,
            skr_variant: "SKR03".into(),
        },
        ChartAccount {
            code: "1576".into(),
            name: "Abziehbare Vorsteuer (Platzhalter — Satz ungeprüft)".into(),
            kind: AccountKind::Tax,
            skr_variant: "SKR03".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocked_entries_are_not_postable() {
        let e = JournalEntryDraft::blocked("j1", "EU-B2B sale outside v1 matrix");
        assert!(!e.is_postable());
    }
}
