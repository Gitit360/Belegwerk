use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub at: DateTime<Utc>,
    pub caller: String,
    pub action: String,
    pub decision: String,
    pub detail: Option<String>,
}

pub trait AuditSink: Send + Sync {
    fn record(&self, event: AuditEvent);
}

#[derive(Debug, Default, Clone)]
pub struct InMemoryAuditSink {
    events: Arc<Mutex<Vec<AuditEvent>>>,
}

impl InMemoryAuditSink {
    pub fn events(&self) -> Vec<AuditEvent> {
        self.events.lock().expect("lock").clone()
    }
}

impl AuditSink for InMemoryAuditSink {
    fn record(&self, event: AuditEvent) {
        self.events.lock().expect("lock").push(event);
    }
}
