use crate::manifest::{PluginManifest, SDK_VERSION};
use crate::protocol::{MessageType, PluginMessage};
use belegwerk_application::actions::KnownAction;
use belegwerk_application::permission::{
    PermissionDecision, PermissionGate, PermissionRequest, ResourceRef, Scope,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HostError {
    #[error("incompatible sdkVersion")]
    IncompatibleSdk,
    #[error("capability not granted: {0}")]
    CapabilityDenied(String),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
}

/// Minimal in-process host: handshake + capability filter + permission gate.
#[derive(Debug)]
pub struct PluginRuntime {
    pub manifest: PluginManifest,
    pub granted_capabilities: Vec<String>,
    gate: PermissionGate,
}

impl PluginRuntime {
    pub fn start(manifest: PluginManifest, gate: PermissionGate) -> Result<Self, HostError> {
        if !manifest.is_compatible() {
            return Err(HostError::IncompatibleSdk);
        }
        // Host filters declared capabilities to an allowlist intersection (stub: all declared).
        let granted_capabilities = manifest.capabilities.clone();
        Ok(Self {
            manifest,
            granted_capabilities,
            gate,
        })
    }

    pub fn handshake(&self) -> PluginMessage {
        PluginMessage {
            v: 1,
            id: uuid::Uuid::new_v4().to_string(),
            msg_type: MessageType::Response,
            method: "plugin.handshake".into(),
            correlation_id: None,
            payload: json!({
                "sdkVersion": SDK_VERSION,
                "capabilities": self.granted_capabilities,
                "pluginId": self.manifest.id,
            }),
            error: None,
        }
    }

    pub fn ensure_capability(&self, cap: &str) -> Result<(), HostError> {
        if self.granted_capabilities.iter().any(|c| c == cap) {
            Ok(())
        } else {
            Err(HostError::CapabilityDenied(cap.into()))
        }
    }

    /// Example: bank read goes through gate; payment always denied.
    pub fn authorize_bank_read(&self) -> Result<(), HostError> {
        self.ensure_capability("bank.transactions.read")?;
        let req = PermissionRequest::for_known(
            KnownAction::BankTransactionsRead,
            ResourceRef {
                kind: "bank".into(),
                id: None,
            },
            Scope {
                label: "plugin".into(),
            },
            "{}",
            &self.manifest.id,
        );
        match self.gate.decide(&req) {
            PermissionDecision::Allow => Ok(()),
            PermissionDecision::Deny { reason } => Err(HostError::PermissionDenied(reason)),
            PermissionDecision::RequireApproval => {
                Err(HostError::PermissionDenied("approval required".into()))
            }
        }
    }

    pub fn authorize_bank_payment_blocked(&self) -> Result<(), HostError> {
        let req = PermissionRequest::for_known(
            KnownAction::BankPaymentCreate,
            ResourceRef {
                kind: "bank".into(),
                id: None,
            },
            Scope {
                label: "plugin".into(),
            },
            "{}",
            &self.manifest.id,
        );
        match self.gate.decide(&req) {
            PermissionDecision::Deny { .. } => Err(HostError::PermissionDenied(
                "bank.payment.create is permanently blocked".into(),
            )),
            _ => Err(HostError::PermissionDenied(
                "bank.payment.create must be denied".into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{PluginKind, PluginPermissions};

    fn sample_manifest() -> PluginManifest {
        PluginManifest {
            id: "belegwerk.connector.moneymoney".into(),
            version: "0.0.1".into(),
            sdk_version: "1".into(),
            kind: PluginKind::BankImport,
            display_name: "MoneyMoney (stub)".into(),
            description: Some("Synthetic stub".into()),
            capabilities: vec!["bank.transactions.read".into()],
            permissions: PluginPermissions {
                network_domains: vec![],
                filesystem: vec![],
                credential_scopes: vec![],
                max_runtime_ms: Some(60_000),
                max_memory_mb: Some(256),
            },
            entrypoint: "stub".into(),
            development: true,
        }
    }

    #[test]
    fn handshake_and_block_payments() {
        let gate = PermissionGate::default();
        let rt = PluginRuntime::start(sample_manifest(), gate).unwrap();
        let hs = rt.handshake();
        assert_eq!(hs.method, "plugin.handshake");
        assert!(rt.authorize_bank_read().is_ok());
        assert!(rt.authorize_bank_payment_blocked().is_err());
    }
}
