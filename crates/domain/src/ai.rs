//! AI provider port — offline-safe; no tool escalation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCompletionRequest {
    pub prompt: String,
    pub schema_name: Option<String>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCompletionResponse {
    pub text: String,
    pub provider: String,
    pub model: String,
    pub cached: bool,
}

pub trait AiProvider: Send + Sync {
    fn id(&self) -> &str;
    fn complete(&self, req: &AiCompletionRequest) -> Result<AiCompletionResponse, AiError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiError {
    Unavailable(String),
    InvalidSchema,
    BudgetExceeded,
}

/// Default offline stub — core remains usable without Ollama.
#[derive(Debug, Default, Clone)]
pub struct NullAiProvider;

impl AiProvider for NullAiProvider {
    fn id(&self) -> &str {
        "null"
    }

    fn complete(&self, _req: &AiCompletionRequest) -> Result<AiCompletionResponse, AiError> {
        Err(AiError::Unavailable(
            "AI disabled; Ollama optional (M08). Core works offline.".into(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct OllamaProvider {
    pub base_url: String,
    pub model: String,
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:11434".into(),
            model: "llama3.2".into(),
        }
    }
}

impl AiProvider for OllamaProvider {
    fn id(&self) -> &str {
        "ollama"
    }

    fn complete(&self, _req: &AiCompletionRequest) -> Result<AiCompletionResponse, AiError> {
        // Network call deferred — interface only until M08 verify.
        Err(AiError::Unavailable(
            "Ollama HTTP client not wired in this scaffold; use NullAiProvider offline".into(),
        ))
    }
}
