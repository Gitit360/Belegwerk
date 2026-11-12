//! Mail search/draft and portal connector stubs (M10). No silent mutations.

use belegwerk_domain::mail::{
    portal_slots_v1, MailDraft, MailHit, MailSearchQuery, PortalConnectorSlot,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalProbeResult {
    pub slot_id: String,
    pub ok: bool,
    pub message: String,
}

pub struct MailPortalService;

impl MailPortalService {
    pub fn search_synthetic(&self, query: &MailSearchQuery) -> Vec<MailHit> {
        let mut hits = vec![MailHit {
            id: "mail-synth-001".into(),
            subject: "Ihre Rechnung INV-SYNTH-1001".into(),
            from: "billing@synthetic.example".into(),
            received_at: "2026-01-12T08:00:00Z".into(),
        }];
        if let Some(ref sub) = query.subject_contains {
            hits.retain(|h| h.subject.to_lowercase().contains(&sub.to_lowercase()));
        }
        hits
    }

    pub fn create_draft(&self, to: Vec<String>, subject: String, body: String) -> MailDraft {
        MailDraft { to, subject, body }
    }

    pub fn send_without_approval(&self) -> Result<(), &'static str> {
        Err("mail.send requires ActionApproval (REQ-S-005)")
    }

    pub fn portal_slots(&self) -> Vec<PortalConnectorSlot> {
        portal_slots_v1()
    }

    pub fn probe_portal(&self, slot_id: &str) -> PortalProbeResult {
        if slot_id == "mobile-de" {
            PortalProbeResult {
                slot_id: slot_id.into(),
                ok: false,
                message: "unconfigured placeholder slot — no fake brand (D-07)".into(),
            }
        } else {
            PortalProbeResult {
                slot_id: slot_id.into(),
                ok: false,
                message: "stub connector — live probe deferred".into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mail_search_and_send_guard() {
        let svc = MailPortalService;
        let hits = svc.search_synthetic(&MailSearchQuery {
            subject_contains: Some("INV-SYNTH".into()),
            from_contains: None,
            since: None,
        });
        assert_eq!(hits.len(), 1);
        assert!(svc.send_without_approval().is_err());
        assert!(!svc.portal_slots().is_empty());
        assert!(!svc.probe_portal("mobile-de").ok);
    }
}
