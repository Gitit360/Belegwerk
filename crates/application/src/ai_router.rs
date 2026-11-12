//! AI router: try Ollama, fall back offline without silent cloud (M08).

use belegwerk_domain::ai::{
    AiCompletionRequest, AiCompletionResponse, AiError, AiProvider, NullAiProvider, OllamaProvider,
};

#[derive(Debug, Clone)]
pub struct AiRouter {
    pub prefer_ollama: bool,
    pub ollama: OllamaProvider,
    pub offline: NullAiProvider,
    /// When true, probe is skipped and offline is used (Linux CI default).
    pub force_offline: bool,
}

impl Default for AiRouter {
    fn default() -> Self {
        Self {
            prefer_ollama: true,
            ollama: OllamaProvider::default(),
            offline: NullAiProvider,
            force_offline: true,
        }
    }
}

impl AiRouter {
    pub fn complete(&self, req: &AiCompletionRequest) -> Result<AiCompletionResponse, AiError> {
        if self.force_offline || !self.prefer_ollama {
            return self.offline.complete(req);
        }
        match self.ollama.complete(req) {
            Ok(r) => Ok(r),
            Err(AiError::Unavailable(_)) => self.offline.complete(req),
            Err(e) => Err(e),
        }
    }

    pub fn active_provider_id(&self) -> &'static str {
        if self.force_offline || !self.prefer_ollama {
            "null"
        } else {
            "ollama"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offline_fallback_keeps_core_usable() {
        let router = AiRouter::default();
        assert_eq!(router.active_provider_id(), "null");
        let err = router
            .complete(&AiCompletionRequest {
                prompt: "classify".into(),
                schema_name: None,
                max_tokens: Some(16),
            })
            .unwrap_err();
        assert!(matches!(err, AiError::Unavailable(_)));
    }
}
