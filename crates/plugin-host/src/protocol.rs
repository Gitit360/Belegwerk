use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Request,
    Response,
    Event,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginMessage {
    pub v: u32,
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: MessageType,
    pub method: String,
    pub correlation_id: Option<String>,
    pub payload: Value,
    pub error: Option<Value>,
}

impl PluginMessage {
    pub fn request(method: impl Into<String>, payload: Value) -> Self {
        Self {
            v: 1,
            id: uuid::Uuid::new_v4().to_string(),
            msg_type: MessageType::Request,
            method: method.into(),
            correlation_id: None,
            payload,
            error: None,
        }
    }
}
