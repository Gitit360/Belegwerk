//! Matching engine skeleton: amount + reference heuristics (M05).

use belegwerk_domain::{
    DocumentId, EconomicTransaction, EconomicTransactionId, MatchCandidate, MatchConfidence,
    MoneyAmount, SourceTransaction,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMatchInput {
    pub document_id: DocumentId,
    pub amount: Option<MoneyAmount>,
    pub invoice_ref: Option<String>,
    pub counterparty: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct MatchingEngine {
    pub amount_tolerance_minor: i64,
}

impl MatchingEngine {
    pub fn suggest_for_economic(
        &self,
        economic: &EconomicTransaction,
        documents: &[DocumentMatchInput],
    ) -> Vec<MatchCandidate> {
        let mut out = Vec::new();
        for doc in documents {
            if let Some(c) = self.score_pair(
                economic.id,
                &economic.amount,
                economic.description.as_deref(),
                doc,
            ) {
                out.push(c);
            }
        }
        out.sort_by_key(|c| match c.confidence {
            MatchConfidence::Exact => 0,
            MatchConfidence::High => 1,
            MatchConfidence::Medium => 2,
            MatchConfidence::Low => 3,
            MatchConfidence::Manual => 4,
        });
        out
    }

    pub fn suggest_for_source(
        &self,
        source: &SourceTransaction,
        documents: &[DocumentMatchInput],
    ) -> Vec<MatchCandidate> {
        // Temporary economic id for candidate pairing until merge.
        let eco_id = EconomicTransactionId::new();
        let mut out = Vec::new();
        for doc in documents {
            if let Some(mut c) =
                self.score_pair(eco_id, &source.amount, source.purpose.as_deref(), doc)
            {
                // Keep reason but note source-driven.
                c.reason = format!("source:{}", c.reason);
                out.push(c);
            }
        }
        out
    }

    fn score_pair(
        &self,
        economic_id: EconomicTransactionId,
        amount: &MoneyAmount,
        purpose: Option<&str>,
        doc: &DocumentMatchInput,
    ) -> Option<MatchCandidate> {
        let mut score_bits = Vec::new();
        let mut confidence = None;

        if let Some(ref inv) = doc.invoice_ref {
            if let Some(p) = purpose {
                if !inv.is_empty() && p.to_uppercase().contains(&inv.to_uppercase()) {
                    score_bits.push(format!("reference_match:{inv}"));
                    confidence = Some(MatchConfidence::High);
                }
            }
        }

        if let Some(ref doc_amount) = doc.amount {
            if doc_amount.currency == amount.currency {
                let delta = (doc_amount.amount_minor.abs() - amount.amount_minor.abs()).abs();
                if delta <= self.amount_tolerance_minor {
                    score_bits.push(format!(
                        "amount_match:{} {}",
                        doc_amount.amount_minor.abs(),
                        doc_amount.currency.0
                    ));
                    confidence = Some(match confidence {
                        Some(MatchConfidence::High) => MatchConfidence::Exact,
                        Some(other) => other,
                        None => MatchConfidence::Medium,
                    });
                }
            }
        }

        confidence.map(|confidence| MatchCandidate {
            economic_transaction_id: economic_id,
            document_id: doc.document_id,
            confidence,
            reason: score_bits.join("+"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use belegwerk_domain::{CalendarDate, CurrencyCode};

    #[test]
    fn matches_reference_and_amount_as_exact() {
        let engine = MatchingEngine::default();
        let source = SourceTransaction {
            id: Default::default(),
            provider_id: "synth".into(),
            source_account_id: "a".into(),
            source_transaction_ref: "t1".into(),
            booking_date: CalendarDate::from_ymd(2026, 1, 15).unwrap(),
            value_date: None,
            amount: MoneyAmount::new(-2499, CurrencyCode::eur()),
            counterparty: Some("Synthetic Cloud Tools LLC".into()),
            purpose: Some("INV-SYNTH-1001 SaaS".into()),
            status: belegwerk_domain::TransactionStatus::Imported,
            imported_at: chrono::Utc::now(),
            raw_ref: None,
        };
        let docs = vec![DocumentMatchInput {
            document_id: Default::default(),
            amount: Some(MoneyAmount::new(2499, CurrencyCode::eur())),
            invoice_ref: Some("INV-SYNTH-1001".into()),
            counterparty: Some("Synthetic Cloud Tools LLC".into()),
        }];
        let hits = engine.suggest_for_source(&source, &docs);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].confidence, MatchConfidence::Exact);
    }

    #[test]
    fn no_match_when_unrelated() {
        let engine = MatchingEngine::default();
        let eco = EconomicTransaction {
            id: Default::default(),
            amount: MoneyAmount::new(-1000, CurrencyCode::eur()),
            booking_date: CalendarDate::from_ymd(2026, 1, 1).unwrap(),
            description: Some("RENT".into()),
            source_transaction_ids: vec![],
            status: belegwerk_domain::TransactionStatus::Imported,
        };
        let docs = vec![DocumentMatchInput {
            document_id: Default::default(),
            amount: Some(MoneyAmount::new(9999, CurrencyCode::eur())),
            invoice_ref: Some("OTHER".into()),
            counterparty: None,
        }];
        assert!(engine.suggest_for_economic(&eco, &docs).is_empty());
    }
}
