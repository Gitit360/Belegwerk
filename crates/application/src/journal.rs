//! Journal draft helpers — balanced lines; unverified tax cases stay blocked (M11).

use belegwerk_domain::accounting::{JournalEntryDraft, JournalLine, TaxRulePackRef};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum JournalError {
    #[error("unbalanced: debit_minor={debit} credit_minor={credit}")]
    Unbalanced { debit: i64, credit: i64 },
    #[error("blocked unsupported case: {0}")]
    Blocked(String),
    #[error("missing reviewed/active rule pack")]
    MissingRulePack,
}

pub fn line_totals(lines: &[JournalLine]) -> (i64, i64) {
    let debit: i64 = lines.iter().map(|l| l.debit_minor).sum();
    let credit: i64 = lines.iter().map(|l| l.credit_minor).sum();
    (debit, credit)
}

pub fn assert_balanced(lines: &[JournalLine]) -> Result<(), JournalError> {
    let (debit, credit) = line_totals(lines);
    if debit != credit {
        return Err(JournalError::Unbalanced { debit, credit });
    }
    Ok(())
}

pub fn draft_expense_against_bank(
    id: impl Into<String>,
    expense_account: &str,
    bank_account: &str,
    amount_minor: i64,
    currency: &str,
    rule_pack: TaxRulePackRef,
) -> Result<JournalEntryDraft, JournalError> {
    if rule_pack.status != "active" && rule_pack.status != "reviewed" {
        return Err(JournalError::MissingRulePack);
    }
    let lines = vec![
        JournalLine {
            account_code: expense_account.into(),
            debit_minor: amount_minor.abs(),
            credit_minor: 0,
            currency: currency.into(),
        },
        JournalLine {
            account_code: bank_account.into(),
            debit_minor: 0,
            credit_minor: amount_minor.abs(),
            currency: currency.into(),
        },
    ];
    assert_balanced(&lines)?;
    Ok(JournalEntryDraft {
        id: id.into(),
        description: Some("synthetic expense draft".into()),
        lines,
        rule_pack: Some(rule_pack),
        blocked_unsupported: false,
        block_reason: None,
    })
}

pub fn block_unverified_tax_case(
    id: impl Into<String>,
    reason: impl Into<String>,
) -> JournalEntryDraft {
    JournalEntryDraft::blocked(id, reason)
}

/// Expense with optional private share (TAX-PRIVATE-SHARE).
/// `private_percent` 0 = fully business; >0 splits debit to expense vs Privatentnahme 1800.
pub fn draft_expense_with_private_share(
    id: impl Into<String>,
    expense_account: &str,
    bank_account: &str,
    amount_minor: i64,
    currency: &str,
    private_percent: u8,
    rule_pack: TaxRulePackRef,
) -> Result<JournalEntryDraft, JournalError> {
    if rule_pack.status != "active" && rule_pack.status != "reviewed" {
        return Err(JournalError::MissingRulePack);
    }
    let split = belegwerk_domain::tax_rules::TaxRulePack::split_private_share(
        amount_minor,
        private_percent,
    )
    .map_err(|e| JournalError::Blocked(e.to_string()))?;
    let mut lines = Vec::new();
    if split.business_minor > 0 {
        lines.push(JournalLine {
            account_code: expense_account.into(),
            debit_minor: split.business_minor,
            credit_minor: 0,
            currency: currency.into(),
        });
    }
    if split.private_minor > 0 {
        lines.push(JournalLine {
            account_code: "1800".into(),
            debit_minor: split.private_minor,
            credit_minor: 0,
            currency: currency.into(),
        });
    }
    lines.push(JournalLine {
        account_code: bank_account.into(),
        debit_minor: 0,
        credit_minor: amount_minor.abs(),
        currency: currency.into(),
    });
    assert_balanced(&lines)?;
    Ok(JournalEntryDraft {
        id: id.into(),
        description: Some(format!(
            "expense draft private_share={private_percent}% (TAX-PRIVATE-SHARE)"
        )),
        lines,
        rule_pack: Some(rule_pack),
        blocked_unsupported: false,
        block_reason: None,
    })
}

pub fn can_post(draft: &JournalEntryDraft) -> Result<(), JournalError> {
    if draft.blocked_unsupported {
        return Err(JournalError::Blocked(
            draft
                .block_reason
                .clone()
                .unwrap_or_else(|| "unsupported".into()),
        ));
    }
    assert_balanced(&draft.lines)?;
    match &draft.rule_pack {
        Some(r) if r.status == "active" || r.status == "reviewed" => Ok(()),
        _ => Err(JournalError::MissingRulePack),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_draft_with_reviewed_pack_is_postable() {
        let pack = TaxRulePackRef {
            id: "core".into(),
            version: "0.0.0-test".into(),
            status: "reviewed".into(),
            source_note: Some("synthetic test pack — not tax advice".into()),
        };
        let draft = draft_expense_against_bank("j-1", "4950", "1200", 2499, "EUR", pack).unwrap();
        assert!(can_post(&draft).is_ok());
        assert!(draft.is_postable());
    }

    #[test]
    fn eu_b2b_case_blocked() {
        let draft = block_unverified_tax_case("j-eu", "EU-B2B sale outside v1 matrix (D-06)");
        assert_eq!(
            can_post(&draft),
            Err(JournalError::Blocked(
                "EU-B2B sale outside v1 matrix (D-06)".into()
            ))
        );
    }

    #[test]
    fn private_share_splits_expense_and_entnahme() {
        let pack = TaxRulePackRef {
            id: "belegwerk-core".into(),
            version: "2026.1.0".into(),
            status: "reviewed".into(),
            source_note: None,
        };
        let draft =
            draft_expense_with_private_share("j-priv", "4950", "1200", 1000, "EUR", 30, pack)
                .unwrap();
        assert!(can_post(&draft).is_ok());
        let priv_line = draft.lines.iter().find(|l| l.account_code == "1800").unwrap();
        assert_eq!(priv_line.debit_minor, 300);
        let exp = draft.lines.iter().find(|l| l.account_code == "4950").unwrap();
        assert_eq!(exp.debit_minor, 700);
    }

    #[test]
    fn unbalanced_rejected() {
        let lines = vec![JournalLine {
            account_code: "4950".into(),
            debit_minor: 100,
            credit_minor: 0,
            currency: "EUR".into(),
        }];
        assert!(matches!(
            assert_balanced(&lines),
            Err(JournalError::Unbalanced { .. })
        ));
    }
}
