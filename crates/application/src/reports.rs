//! Tax vs management report shells with drill-down stubs (M12).

use belegwerk_domain::reporting::{ManagementReportShell, TaxReturnDraft};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportLine {
    pub label: String,
    pub amount_minor: i64,
    pub currency: String,
    pub source_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportShellView {
    pub id: String,
    pub title: String,
    pub basis: String,
    pub lines: Vec<ReportLine>,
    pub note: String,
}

pub fn tax_draft_shell(kind: &str, period: &str) -> TaxReturnDraft {
    TaxReturnDraft::shell(kind, period)
}

pub fn management_cash_shell() -> ReportShellView {
    let shell = ManagementReportShell::cash_overview();
    ReportShellView {
        id: shell.id,
        title: shell.title,
        basis: shell.basis,
        lines: vec![
            ReportLine {
                label: "Synthetische Ausgaben (Fixture)".into(),
                amount_minor: -2499,
                currency: "EUR".into(),
                source_ref: Some("eco-synth-001".into()),
            },
            ReportLine {
                label: "Synthetische Einnahmen GT (Fixture)".into(),
                amount_minor: 119000,
                currency: "EUR".into(),
                source_ref: Some("GT-SYNTH-2001".into()),
            },
        ],
        note: "Management basis — not a tax filing. Drill-down refs are synthetic.".into(),
    }
}

pub fn drill_down(source_ref: &str) -> Option<String> {
    match source_ref {
        "eco-synth-001" => {
            Some("EconomicTransaction eco-synth-001 → SourceTransaction mm-synth-001".into())
        }
        "GT-SYNTH-2001" => Some("GrandTotal invoice GT-SYNTH-2001 (synthetic fixture)".into()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn management_shell_has_drilldowns() {
        let view = management_cash_shell();
        assert_eq!(view.basis, "management");
        let href = view.lines[0].source_ref.as_deref().unwrap();
        assert!(drill_down(href).unwrap().contains("mm-synth-001"));
        let draft = tax_draft_shell("ustva", "2026-01");
        assert_eq!(draft.status, "draft_shell");
    }
}
