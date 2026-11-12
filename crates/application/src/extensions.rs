//! M15 extension hook registry — stubs only.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExtensionId {
    BankX,
    Steuerboard,
    EricElster,
    EuB2bSales,
    MobileDePortal,
    GraphUi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionHook {
    pub id: ExtensionId,
    pub display_name: String,
    pub status: String,
    pub blocked_reason: Option<String>,
}

pub fn extension_registry() -> Vec<ExtensionHook> {
    vec![
        ExtensionHook {
            id: ExtensionId::BankX,
            display_name: "Bank X Professional".into(),
            status: "phase2_stub".into(),
            blocked_reason: Some("Interface TBD after MoneyMoney (D-03)".into()),
        },
        ExtensionHook {
            id: ExtensionId::Steuerboard,
            display_name: "Steuerboard".into(),
            status: "post_m15".into(),
            blocked_reason: Some("Use generic export/delivery first (D-13)".into()),
        },
        ExtensionHook {
            id: ExtensionId::EricElster,
            display_name: "ERiC / ELSTER Direkt".into(),
            status: "deferred".into(),
            blocked_reason: Some("Developer registration + platform check required".into()),
        },
        ExtensionHook {
            id: ExtensionId::EuB2bSales,
            display_name: "EU-B2B Verkäufe".into(),
            status: "blocked_v1".into(),
            blocked_reason: Some("Outside v1 case matrix (D-06)".into()),
        },
        ExtensionHook {
            id: ExtensionId::MobileDePortal,
            display_name: "Mobilfunk-Portal".into(),
            status: "slot".into(),
            blocked_reason: Some("Placeholder id mobile-de only (D-07)".into()),
        },
        ExtensionHook {
            id: ExtensionId::GraphUi,
            display_name: "Node-Graph UI".into(),
            status: "deferred".into(),
            blocked_reason: Some("Relational lists first (D-17)".into()),
        },
    ]
}

pub fn is_enabled(id: ExtensionId) -> bool {
    // Nothing enabled in v1 overnight scaffold.
    let _ = id;
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_lists_blocked_extensions() {
        let reg = extension_registry();
        assert!(reg.iter().any(|e| e.id == ExtensionId::EuB2bSales));
        assert!(!is_enabled(ExtensionId::BankX));
    }
}
