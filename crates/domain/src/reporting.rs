use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxReturnDraft {
    pub id: String,
    pub tax_kind: String,
    pub period: String,
    pub status: String,
    pub form_version: Option<String>,
    pub notes: Option<String>,
}

impl TaxReturnDraft {
    pub fn shell(tax_kind: &str, period: &str) -> Self {
        Self {
            id: format!("{tax_kind}-{period}"),
            tax_kind: tax_kind.into(),
            period: period.into(),
            status: "draft_shell".into(),
            form_version: None,
            notes: Some(
                "Form field mapping blocked until ELSTER/BMF sources verified (M12)".into(),
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagementReportShell {
    pub id: String,
    pub title: String,
    /// `tax` or `management` — never mixed silently.
    pub basis: String,
}

impl ManagementReportShell {
    pub fn cash_overview() -> Self {
        Self {
            id: "mgmt-cash-overview".into(),
            title: "Liquiditätsübersicht (Shell)".into(),
            basis: "management".into(),
        }
    }
}
