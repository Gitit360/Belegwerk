//! Rebuild match_candidates via MatchingEngine from economic txs + document extracts.

use crate::extraction_service::ExtractionService;
use crate::matching_engine::{DocumentMatchInput, MatchingEngine};
use belegwerk_domain::{
    CalendarDate, ContentHash, CurrencyCode, DocumentId, EconomicTransaction,
    EconomicTransactionId, MatchConfidence, MoneyAmount, TransactionStatus,
};
use belegwerk_infrastructure::{migrate, open_db, BlobStore, DataPaths, FsBlobStore};
use chrono::NaiveDate;
use rusqlite::Connection;
use std::fs;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum MatchRebuildError {
    #[error("{0}")]
    Storage(String),
}

pub struct MatchRebuildService {
    pub paths: DataPaths,
}

impl MatchRebuildService {
    pub fn rebuild_all(&self) -> Result<usize, MatchRebuildError> {
        self.paths
            .ensure()
            .map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
        let conn = open_db(&self.paths.db_path())
            .map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
        let store = FsBlobStore::new(self.paths.clone())
            .map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
        let extraction = ExtractionService {
            paths: self.paths.clone(),
        };

        let ecos = load_economics(&conn)?;
        let docs = load_document_inputs(&conn, &store, &extraction)?;
        let engine = MatchingEngine::default();

        conn.execute("DELETE FROM match_candidates", [])
            .map_err(|e| MatchRebuildError::Storage(e.to_string()))?;

        let mut n = 0usize;
        for eco in &ecos {
            for cand in engine.suggest_for_economic(eco, &docs) {
                conn.execute(
                    "INSERT INTO match_candidates(economic_transaction_id, document_id, confidence, reason)
                     VALUES (?1, ?2, ?3, ?4)",
                    (
                        cand.economic_transaction_id.to_string(),
                        cand.document_id.to_string(),
                        confidence_str(cand.confidence),
                        &cand.reason,
                    ),
                )
                .map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
                n += 1;
            }
        }
        Ok(n)
    }
}

fn load_economics(conn: &Connection) -> Result<Vec<EconomicTransaction>, MatchRebuildError> {
    let mut stmt = conn
        .prepare(
            "SELECT id, amount_minor, currency, booking_date, COALESCE(description,''), status
             FROM economic_transactions",
        )
        .map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, String>(5)?,
            ))
        })
        .map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        let (id, amount, currency, date, desc, status) =
            row.map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
        let uuid = Uuid::parse_str(&id).map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
        let booking = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .ok()
            .map(CalendarDate)
            .or_else(|| CalendarDate::from_ymd(2026, 1, 1))
            .ok_or_else(|| MatchRebuildError::Storage(format!("bad date {date}")))?;
        let _ = status;
        out.push(EconomicTransaction {
            id: EconomicTransactionId(uuid),
            amount: MoneyAmount::new(amount, CurrencyCode(currency)),
            booking_date: booking,
            description: Some(desc),
            source_transaction_ids: vec![],
            status: TransactionStatus::Imported,
        });
    }
    Ok(out)
}

fn confidence_str(c: MatchConfidence) -> &'static str {
    match c {
        MatchConfidence::Exact => "exact",
        MatchConfidence::High => "high",
        MatchConfidence::Medium => "medium",
        MatchConfidence::Low => "low",
        MatchConfidence::Manual => "manual",
    }
}

fn load_document_inputs(
    conn: &Connection,
    store: &FsBlobStore,
    extraction: &ExtractionService,
) -> Result<Vec<DocumentMatchInput>, MatchRebuildError> {
    let mut stmt = conn
        .prepare("SELECT id, content_hash, COALESCE(original_filename,'') FROM documents")
        .map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
            ))
        })
        .map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        let (id, hash, filename) = row.map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
        let uuid = Uuid::parse_str(&id).map_err(|e| MatchRebuildError::Storage(e.to_string()))?;
        let ch = ContentHash(hash.clone());
        let text = fs::read(store.path_for(&ch))
            .ok()
            .and_then(|b| String::from_utf8(b).ok())
            .unwrap_or_default();
        if !text.is_empty() {
            let _ = extraction.index_document_text(&hash, &text);
        }
        let extracted = extraction.extract_labeled_text(
            &hash,
            &text,
            &["Invoice", "Amount", "EUR", "Total", "Rechnungsnummer"],
        );
        let invoice_ref = extracted
            .fields
            .iter()
            .find(|f| {
                f.name.eq_ignore_ascii_case("Invoice")
                    || f.name.eq_ignore_ascii_case("Rechnungsnummer")
            })
            .and_then(|f| f.value.clone())
            .or_else(|| scrape_invoice_ref(&text))
            .or_else(|| scrape_invoice_from_packed_filename(&filename));
        let amount = scrape_amount_minor(&text)
            .or_else(|| scrape_amount_from_packed_filename(&filename))
            .map(|m| MoneyAmount::new(m, CurrencyCode::eur()));
        out.push(DocumentMatchInput {
            document_id: DocumentId(uuid),
            amount,
            invoice_ref,
            counterparty: None,
        });
    }
    Ok(out)
}

fn scrape_invoice_ref(text: &str) -> Option<String> {
    for line in text.lines() {
        let u = line.to_uppercase();
        if let Some(idx) = u.find("INV-") {
            let rest = &line[idx..];
            let token: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                .collect();
            if token.len() > 4 {
                return Some(token);
            }
        }
    }
    None
}

fn scrape_amount_minor(text: &str) -> Option<i64> {
    // "Amount: EUR 24.99" or "EUR 24,99"
    for line in text.lines() {
        let u = line.to_uppercase();
        if !(u.contains("AMOUNT") || u.contains("EUR") || u.contains("TOTAL")) {
            continue;
        }
        let digits: String = line
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == ',' || *c == '.')
            .collect();
        let norm = digits.replace(',', ".");
        if let Ok(v) = norm.parse::<f64>() {
            return Some((v * 100.0).round() as i64);
        }
    }
    None
}

/// Seed/fixture packed filename: `name|amount|inv|excerpt`
fn scrape_invoice_from_packed_filename(packed: &str) -> Option<String> {
    let parts: Vec<_> = packed.split('|').collect();
    if parts.len() >= 3 && !parts[2].is_empty() {
        Some(parts[2].to_string())
    } else {
        None
    }
}

fn scrape_amount_from_packed_filename(packed: &str) -> Option<i64> {
    let parts: Vec<_> = packed.split('|').collect();
    if parts.len() >= 2 {
        parts[1].parse().ok()
    } else {
        None
    }
}
