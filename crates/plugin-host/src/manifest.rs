use serde::{Deserialize, Serialize};

pub const SDK_VERSION: &str = "1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkVersion(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginKind {
    BankImport,
    InvoiceImport,
    DocumentSource,
    Export,
    Portal,
    Mail,
    Ocr,
    AiProvider,
    Reporting,
    Delivery,
    TaxRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginPermissions {
    #[serde(default)]
    pub network_domains: Vec<String>,
    #[serde(default)]
    pub filesystem: Vec<String>,
    #[serde(default)]
    pub credential_scopes: Vec<String>,
    pub max_runtime_ms: Option<u64>,
    pub max_memory_mb: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    pub sdk_version: String,
    pub kind: PluginKind,
    pub display_name: String,
    pub description: Option<String>,
    pub capabilities: Vec<String>,
    pub permissions: PluginPermissions,
    pub entrypoint: String,
    #[serde(default)]
    pub development: bool,
}

impl PluginManifest {
    pub fn is_compatible(&self) -> bool {
        self.sdk_version == SDK_VERSION || self.sdk_version == "1"
    }
}
