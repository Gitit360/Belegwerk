use belegwerk_application::actions::KnownAction;
use belegwerk_application::permission::{
    PermissionDecision, PermissionGate, PermissionRequest, ResourceRef, Scope,
};
use belegwerk_plugin_host::manifest::{PluginKind, PluginManifest, PluginPermissions};
use belegwerk_plugin_host::PluginRuntime;

#[test]
fn ui_cannot_create_bank_payment_even_if_requested() {
    let gate = PermissionGate::default();
    let req = PermissionRequest::for_known(
        KnownAction::BankPaymentCreate,
        ResourceRef {
            kind: "account".into(),
            id: Some("synth".into()),
        },
        Scope { label: "ui".into() },
        "{\"amount\":1}",
        "desktop-ui",
    );
    assert!(matches!(gate.decide(&req), PermissionDecision::Deny { .. }));
}

#[test]
fn plugin_runtime_respects_gate_for_payments() {
    let manifest = PluginManifest {
        id: "belegwerk.connector.moneymoney".into(),
        version: "0.0.1".into(),
        sdk_version: "1".into(),
        kind: PluginKind::BankImport,
        display_name: "MoneyMoney".into(),
        description: None,
        capabilities: vec!["bank.transactions.read".into()],
        permissions: PluginPermissions {
            network_domains: vec![],
            filesystem: vec![],
            credential_scopes: vec![],
            max_runtime_ms: None,
            max_memory_mb: None,
        },
        entrypoint: "stub".into(),
        development: true,
    };
    let rt = PluginRuntime::start(manifest, PermissionGate::default()).unwrap();
    assert!(rt.authorize_bank_read().is_ok());
    assert!(rt.authorize_bank_payment_blocked().is_err());
}
