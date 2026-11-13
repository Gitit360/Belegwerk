//! Confirm/reject match suggestions, persist PaymentAllocation, batch preview (M05).

use crate::actions::KnownAction;
use crate::permission::{PermissionGate, PermissionRequest, ResourceRef, Scope};
use belegwerk_domain::ids::{DocumentId, EconomicTransactionId};
use belegwerk_domain::matching::{
    open_remainder, BatchPreview, MatchConfidence, MatchDecision, MatchDecisionKind,
    PaymentAllocation,
};
use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use chrono::Utc;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum MatchWorkflowError {
    #[error("permission denied: {0}")]
    Denied(String),
    #[error("{0}")]
    Storage(String),
    #[error("{0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkbenchTx {
    pub id: String,
    pub booking_date: String,
    pub amount_minor: i64,
    pub currency: String,
    pub description: String,
    pub counterparty: String,
    pub status: String,
    pub allocated_minor: i64,
    pub open_remainder_minor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkbenchDoc {
    pub id: String,
    pub filename: String,
    pub invoice_ref: String,
    pub amount_minor: i64,
    pub currency: String,
    pub excerpt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkbenchSuggestion {
    pub transaction_id: String,
    pub document_id: String,
    pub confidence: String,
    pub reason: String,
    pub decision: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkbenchSnapshot {
    pub note: String,
    pub transactions: Vec<WorkbenchTx>,
    pub documents: Vec<WorkbenchDoc>,
    pub suggestions: Vec<WorkbenchSuggestion>,
    pub allocations: Vec<PaymentAllocationDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAllocationDto {
    pub id: String,
    pub economic_transaction_id: String,
    pub document_id: String,
    pub allocated_minor: i64,
    pub currency: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchItem {
    pub transaction_id: String,
    pub document_id: String,
}

pub struct MatchingWorkflow<'a> {
    pub gate: &'a PermissionGate,
    pub paths: DataPaths,
    pub caller: String,
}

impl MatchingWorkflow<'_> {
    fn conn(&self) -> Result<Connection, MatchWorkflowError> {
        self.paths
            .ensure()
            .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
        let conn = open_db(&self.paths.db_path())
            .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
        migrate(&conn).map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
        Ok(conn)
    }

    fn authorize(&self, action: KnownAction, detail: &str) -> Result<(), MatchWorkflowError> {
        let req = PermissionRequest::for_known(
            action,
            ResourceRef {
                kind: "match".into(),
                id: None,
            },
            Scope {
                label: "matching".into(),
            },
            detail,
            &self.caller,
        );
        self.gate
            .authorize(&req)
            .map_err(|e| MatchWorkflowError::Denied(e.to_string()))
    }

    /// Seed synthetic economic txs, docs, and suggestions if empty (dev/demo).
    pub fn seed_synthetic_if_empty(&self) -> Result<(), MatchWorkflowError> {
        let conn = self.conn()?;
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM economic_transactions", [], |r| {
                r.get(0)
            })
            .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
        if n > 0 {
            return Ok(());
        }
        seed_rows(&conn).map_err(|e| MatchWorkflowError::Storage(e.to_string()))
    }

    pub fn snapshot(&self) -> Result<WorkbenchSnapshot, MatchWorkflowError> {
        self.seed_synthetic_if_empty()?;
        let conn = self.conn()?;
        let transactions = list_txs(&conn)?;
        let documents = list_docs(&conn)?;
        let suggestions = list_suggestions(&conn)?;
        let allocations = list_allocations(&conn)?;
        Ok(WorkbenchSnapshot {
            note: "synthetic only — not real bank or invoice data".into(),
            transactions,
            documents,
            suggestions,
            allocations,
        })
    }

    pub fn confirm(
        &self,
        economic_transaction_id: &str,
        document_id: &str,
        allocated_minor: Option<i64>,
    ) -> Result<PaymentAllocation, MatchWorkflowError> {
        self.authorize(
            KnownAction::MatchConfirm,
            &format!("confirm:{economic_transaction_id}:{document_id}"),
        )?;
        let conn = self.conn()?;
        let eco_id = resolve_id(&conn, economic_transaction_id);
        let doc_id = resolve_id(&conn, document_id);
        let (amount_minor, currency) = conn
            .query_row(
                "SELECT amount_minor, currency FROM economic_transactions WHERE id = ?1",
                [&eco_id],
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)),
            )
            .map_err(|_| {
                MatchWorkflowError::Invalid(format!(
                    "unknown transaction {economic_transaction_id}"
                ))
            })?;
        let already: i64 = conn
            .query_row(
                "SELECT COALESCE(SUM(allocated_minor), 0) FROM payment_allocations WHERE economic_transaction_id = ?1",
                [&eco_id],
                |r| r.get(0),
            )
            .unwrap_or(0);
        let alloc = allocated_minor.unwrap_or_else(|| open_remainder(amount_minor, already));
        if alloc <= 0 {
            return Err(MatchWorkflowError::Invalid(
                "nothing left to allocate (remainder is 0)".into(),
            ));
        }
        if alloc > open_remainder(amount_minor, already) {
            return Err(MatchWorkflowError::Invalid(
                "allocation exceeds open remainder".into(),
            ));
        }
        let (confidence, reason) = suggestion_meta(&conn, economic_transaction_id, document_id)?;
        let decision = MatchDecision {
            id: Uuid::new_v4(),
            economic_transaction_id: parse_eco(&eco_id)?,
            document_id: parse_doc(&doc_id)?,
            decision: MatchDecisionKind::Confirmed,
            confidence,
            reason: reason.clone(),
            decided_at: Utc::now(),
        };
        upsert_decision(&conn, &decision)?;
        let payment = PaymentAllocation {
            id: Uuid::new_v4(),
            economic_transaction_id: decision.economic_transaction_id,
            document_id: decision.document_id,
            allocated_minor: alloc,
            currency,
            created_at: Utc::now(),
        };
        conn.execute(
            "INSERT INTO payment_allocations(id, economic_transaction_id, document_id, allocated_minor, currency, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                payment.id.to_string(),
                payment.economic_transaction_id.to_string(),
                payment.document_id.to_string(),
                payment.allocated_minor,
                &payment.currency,
                payment.created_at.to_rfc3339(),
            ),
        )
        .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
        Ok(payment)
    }

    pub fn reject(
        &self,
        economic_transaction_id: &str,
        document_id: &str,
    ) -> Result<MatchDecision, MatchWorkflowError> {
        self.authorize(
            KnownAction::MatchReject,
            &format!("reject:{economic_transaction_id}:{document_id}"),
        )?;
        let conn = self.conn()?;
        let eco_id = resolve_id(&conn, economic_transaction_id);
        let doc_id = resolve_id(&conn, document_id);
        let (confidence, reason) = suggestion_meta(&conn, economic_transaction_id, document_id)?;
        let decision = MatchDecision {
            id: Uuid::new_v4(),
            economic_transaction_id: parse_eco(&eco_id)?,
            document_id: parse_doc(&doc_id)?,
            decision: MatchDecisionKind::Rejected,
            confidence,
            reason,
            decided_at: Utc::now(),
        };
        upsert_decision(&conn, &decision)?;
        Ok(decision)
    }

    pub fn batch_preview(&self, items: &[BatchItem]) -> Result<BatchPreview, MatchWorkflowError> {
        let conn = self.conn()?;
        let mut confirmable = 0usize;
        let mut exceptions = Vec::new();
        let mut sum_allocated = 0i64;
        let mut sum_remainder = 0i64;
        for item in items {
            match preview_one(&conn, &item.transaction_id, &item.document_id) {
                Ok((alloc, rem, ok, exc)) => {
                    if ok {
                        confirmable += 1;
                        sum_allocated += alloc;
                        sum_remainder += rem;
                    } else if let Some(e) = exc {
                        exceptions.push(e);
                    }
                }
                Err(e) => exceptions.push(e),
            }
        }
        Ok(BatchPreview {
            selected_count: items.len(),
            confirmable_count: confirmable,
            exception_count: exceptions.len(),
            sum_allocated_minor: sum_allocated,
            sum_open_remainder_minor: sum_remainder,
            exceptions,
        })
    }
}

fn parse_eco(id: &str) -> Result<EconomicTransactionId, MatchWorkflowError> {
    let uuid = Uuid::parse_str(id).unwrap_or_else(|_| synthetic_uuid(id));
    Ok(EconomicTransactionId(uuid))
}

fn parse_doc(id: &str) -> Result<DocumentId, MatchWorkflowError> {
    let uuid = Uuid::parse_str(id).unwrap_or_else(|_| synthetic_uuid(id));
    Ok(DocumentId(uuid))
}

fn synthetic_uuid(label: &str) -> Uuid {
    // Deterministic UUID from label for fixture string ids like eco-synth-001.
    let bytes = {
        use sha2::{Digest, Sha256};
        let dig = Sha256::digest(label.as_bytes());
        let mut b = [0u8; 16];
        b.copy_from_slice(&dig[..16]);
        b[6] = (b[6] & 0x0f) | 0x40;
        b[8] = (b[8] & 0x3f) | 0x80;
        b
    };
    Uuid::from_bytes(bytes)
}

fn seed_rows(conn: &Connection) -> Result<(), rusqlite::Error> {
    let ecos = [
        (
            "eco-synth-001",
            -2499i64,
            "2026-01-15",
            "INV-SYNTH-1001 SaaS",
            "Synthetic Cloud Tools LLC",
        ),
        (
            "eco-synth-002",
            -8990,
            "2026-01-18",
            "Office supplies SYNTH",
            "Synthetic Office GmbH",
        ),
    ];
    for (id, amount, date, desc, cp) in ecos {
        let uid = synthetic_uuid(id);
        conn.execute(
            "INSERT INTO economic_transactions(id, amount_minor, currency, booking_date, description, status)
             VALUES (?1, ?2, 'EUR', ?3, ?4, 'imported')",
            (uid.to_string(), amount, date, format!("{desc} [{cp}]")),
        )?;
        // Keep human label mapping via description prefix; also store alias in match_candidates using string form through documents.
        let _ = id;
    }
    // documents need import_batch first
    let batch = synthetic_uuid("batch-synth-match");
    conn.execute(
        "INSERT INTO import_batches(id, source_label, imported_at, note) VALUES (?1, 'matching-fixture', datetime('now'), 'synth')",
        [batch.to_string()],
    )?;
    let docs = [
        (
            "doc-synth-001",
            "synth-invoice.txt",
            "INV-SYNTH-1001",
            2499i64,
            "SYNTHETIC INVOICE — NOT A REAL BELEG\nInvoice: INV-SYNTH-1001\nAmount: EUR 24.99",
        ),
        (
            "doc-synth-002",
            "synth-office.txt",
            "OFF-900",
            5000,
            "SYNTHETIC — unrelated amount for low/no match demo",
        ),
    ];
    for (id, filename, _inv, amount, excerpt) in docs {
        let uid = synthetic_uuid(id);
        // store amount/ref in original_filename field suffix for demo; also hash placeholder
        conn.execute(
            "INSERT INTO documents(id, content_hash, mime_type, original_filename, byte_size, import_batch_id, imported_at, document_date, is_placeholder)
             VALUES (?1, ?2, 'text/plain', ?3, ?4, ?5, datetime('now'), NULL, 0)",
            (
                uid.to_string(),
                format!("synth-{}", id),
                format!("{filename}|{amount}|{_inv}|{excerpt}"),
                excerpt.len() as i64,
                batch.to_string(),
            ),
        )?;
        let _ = amount;
    }
    conn.execute(
        "INSERT INTO match_candidates(economic_transaction_id, document_id, confidence, reason)
         VALUES (?1, ?2, 'exact', 'reference_match:INV-SYNTH-1001+amount_match:2499 EUR')",
        (
            synthetic_uuid("eco-synth-001").to_string(),
            synthetic_uuid("doc-synth-001").to_string(),
        ),
    )?;
    // Alias table via settings for UI string ids
    conn.execute(
        "INSERT OR REPLACE INTO settings_kv(key, value_json, updated_at) VALUES ('fixture_id_map', ?1, datetime('now'))",
        [serde_json::json!({
            "eco-synth-001": synthetic_uuid("eco-synth-001").to_string(),
            "eco-synth-002": synthetic_uuid("eco-synth-002").to_string(),
            "doc-synth-001": synthetic_uuid("doc-synth-001").to_string(),
            "doc-synth-002": synthetic_uuid("doc-synth-002").to_string(),
        })
        .to_string()],
    )?;
    Ok(())
}

fn list_txs(conn: &Connection) -> Result<Vec<WorkbenchTx>, MatchWorkflowError> {
    let map = id_map(conn);
    let mut stmt = conn
        .prepare(
            "SELECT id, booking_date, amount_minor, currency, COALESCE(description,''), status FROM economic_transactions ORDER BY booking_date",
        )
        .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, String>(5)?,
            ))
        })
        .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        let (id, date, amount, currency, desc, status) =
            row.map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
        let allocated: i64 = conn
            .query_row(
                "SELECT COALESCE(SUM(allocated_minor),0) FROM payment_allocations WHERE economic_transaction_id = ?1",
                [&id],
                |r| r.get(0),
            )
            .unwrap_or(0);
        let (counterparty, description) = split_desc(&desc);
        out.push(WorkbenchTx {
            id: reverse_map(&map, &id).unwrap_or(id),
            booking_date: date,
            amount_minor: amount,
            currency,
            description,
            counterparty,
            status,
            allocated_minor: allocated,
            open_remainder_minor: open_remainder(amount, allocated),
        });
    }
    Ok(out)
}

fn list_docs(conn: &Connection) -> Result<Vec<WorkbenchDoc>, MatchWorkflowError> {
    let map = id_map(conn);
    let mut stmt = conn
        .prepare(
            "SELECT id, COALESCE(original_filename,''), byte_size FROM documents ORDER BY imported_at",
        )
        .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
            ))
        })
        .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        let (id, packed, _size) = row.map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
        let (filename, amount, inv, excerpt) = unpack_doc(&packed);
        out.push(WorkbenchDoc {
            id: reverse_map(&map, &id).unwrap_or(id),
            filename,
            invoice_ref: inv,
            amount_minor: amount,
            currency: "EUR".into(),
            excerpt,
        });
    }
    Ok(out)
}

fn list_suggestions(conn: &Connection) -> Result<Vec<WorkbenchSuggestion>, MatchWorkflowError> {
    let map = id_map(conn);
    let mut stmt = conn
        .prepare(
            "SELECT economic_transaction_id, document_id, confidence, reason FROM match_candidates",
        )
        .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            ))
        })
        .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        let (eco, doc, conf, reason) =
            row.map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
        let decision: Option<String> = conn
            .query_row(
                "SELECT decision FROM match_decisions WHERE economic_transaction_id = ?1 AND document_id = ?2",
                [&eco, &doc],
                |r| r.get(0),
            )
            .ok();
        out.push(WorkbenchSuggestion {
            transaction_id: reverse_map(&map, &eco).unwrap_or(eco),
            document_id: reverse_map(&map, &doc).unwrap_or(doc),
            confidence: conf,
            reason,
            decision,
        });
    }
    Ok(out)
}

fn list_allocations(conn: &Connection) -> Result<Vec<PaymentAllocationDto>, MatchWorkflowError> {
    let map = id_map(conn);
    let mut stmt = conn
        .prepare(
            "SELECT id, economic_transaction_id, document_id, allocated_minor, currency, created_at FROM payment_allocations",
        )
        .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
    let rows = stmt
        .query_map([], |r| {
            Ok(PaymentAllocationDto {
                id: r.get(0)?,
                economic_transaction_id: r.get(1)?,
                document_id: r.get(2)?,
                allocated_minor: r.get(3)?,
                currency: r.get(4)?,
                created_at: r.get(5)?,
            })
        })
        .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
    let mut out = Vec::new();
    for row in rows {
        let mut a = row.map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
        a.economic_transaction_id =
            reverse_map(&map, &a.economic_transaction_id).unwrap_or(a.economic_transaction_id);
        a.document_id = reverse_map(&map, &a.document_id).unwrap_or(a.document_id);
        out.push(a);
    }
    Ok(out)
}

fn suggestion_meta(
    conn: &Connection,
    eco_label: &str,
    doc_label: &str,
) -> Result<(MatchConfidence, String), MatchWorkflowError> {
    let eco = resolve_id(conn, eco_label);
    let doc = resolve_id(conn, doc_label);
    let row: Result<(String, String), _> = conn.query_row(
        "SELECT confidence, reason FROM match_candidates WHERE economic_transaction_id = ?1 AND document_id = ?2",
        [&eco, &doc],
        |r| Ok((r.get(0)?, r.get(1)?)),
    );
    match row {
        Ok((c, reason)) => Ok((parse_confidence(&c), reason)),
        Err(_) => Ok((MatchConfidence::Manual, "manual".into())),
    }
}

fn upsert_decision(conn: &Connection, d: &MatchDecision) -> Result<(), MatchWorkflowError> {
    conn.execute(
        "INSERT INTO match_decisions(id, economic_transaction_id, document_id, decision, confidence, reason, decided_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(economic_transaction_id, document_id) DO UPDATE SET
           decision=excluded.decision,
           confidence=excluded.confidence,
           reason=excluded.reason,
           decided_at=excluded.decided_at,
           id=excluded.id",
        (
            d.id.to_string(),
            d.economic_transaction_id.to_string(),
            d.document_id.to_string(),
            match d.decision {
                MatchDecisionKind::Confirmed => "confirmed",
                MatchDecisionKind::Rejected => "rejected",
            },
            format!("{:?}", d.confidence).to_lowercase(),
            &d.reason,
            d.decided_at.to_rfc3339(),
        ),
    )
    .map_err(|e| MatchWorkflowError::Storage(e.to_string()))?;
    Ok(())
}

fn preview_one(
    conn: &Connection,
    eco_label: &str,
    doc_label: &str,
) -> Result<(i64, i64, bool, Option<String>), String> {
    let eco = resolve_id(conn, eco_label);
    let doc = resolve_id(conn, doc_label);
    let (amount, currency): (i64, String) = conn
        .query_row(
            "SELECT amount_minor, currency FROM economic_transactions WHERE id = ?1",
            [&eco],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|_| format!("unbekannte Transaktion {eco_label}"))?;
    let packed: String = conn
        .query_row(
            "SELECT COALESCE(original_filename,'') FROM documents WHERE id = ?1",
            [&doc],
            |r| r.get(0),
        )
        .map_err(|_| format!("unbekanntes Dokument {doc_label}"))?;
    let (_fn, doc_amount, _inv, _ex) = unpack_doc(&packed);
    if currency != "EUR" {
        return Ok((
            0,
            0,
            false,
            Some(format!("Währungs-Ausnahme {eco_label}: {currency}")),
        ));
    }
    let decided: Option<String> = conn
        .query_row(
            "SELECT decision FROM match_decisions WHERE economic_transaction_id = ?1 AND document_id = ?2",
            [&eco, &doc],
            |r| r.get(0),
        )
        .ok();
    if decided.as_deref() == Some("rejected") {
        return Ok((
            0,
            0,
            false,
            Some(format!("bereits abgelehnt: {eco_label}/{doc_label}")),
        ));
    }
    if decided.as_deref() == Some("confirmed") {
        return Ok((
            0,
            0,
            false,
            Some(format!("bereits bestätigt: {eco_label}/{doc_label}")),
        ));
    }
    let allocated: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(allocated_minor),0) FROM payment_allocations WHERE economic_transaction_id = ?1",
            [&eco],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let rem = open_remainder(amount, allocated);
    if rem == 0 {
        return Ok((0, 0, false, Some(format!("kein offener Rest: {eco_label}"))));
    }
    if doc_amount > 0 && (doc_amount - amount.abs()).abs() > rem && doc_amount != amount.abs() {
        // amount mismatch beyond remainder window — still confirmable as exception note
        return Ok((
            rem.min(doc_amount),
            0,
            true,
            Some(format!(
                "Betragsdifferenz {eco_label}: Zahlung {} vs Beleg {doc_amount}",
                amount.abs()
            )),
        ));
    }
    let alloc = rem.min(if doc_amount > 0 { doc_amount } else { rem });
    Ok((alloc, rem - alloc, true, None))
}

fn parse_confidence(s: &str) -> MatchConfidence {
    match s.to_lowercase().as_str() {
        "exact" => MatchConfidence::Exact,
        "high" => MatchConfidence::High,
        "medium" => MatchConfidence::Medium,
        "low" => MatchConfidence::Low,
        _ => MatchConfidence::Manual,
    }
}

fn split_desc(desc: &str) -> (String, String) {
    if let Some((d, rest)) = desc.rsplit_once(" [") {
        if let Some(cp) = rest.strip_suffix(']') {
            return (cp.to_string(), d.to_string());
        }
    }
    ("—".into(), desc.to_string())
}

fn unpack_doc(packed: &str) -> (String, i64, String, String) {
    let parts: Vec<&str> = packed.splitn(4, '|').collect();
    if parts.len() == 4 {
        (
            parts[0].to_string(),
            parts[1].parse().unwrap_or(0),
            parts[2].to_string(),
            parts[3].to_string(),
        )
    } else {
        (packed.to_string(), 0, String::new(), String::new())
    }
}

fn id_map(conn: &Connection) -> serde_json::Map<String, serde_json::Value> {
    let json: String = conn
        .query_row(
            "SELECT value_json FROM settings_kv WHERE key = 'fixture_id_map'",
            [],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| "{}".into());
    serde_json::from_str(&json).unwrap_or_default()
}

fn reverse_map(map: &serde_json::Map<String, serde_json::Value>, uuid: &str) -> Option<String> {
    for (k, v) in map {
        if v.as_str() == Some(uuid) {
            return Some(k.clone());
        }
    }
    None
}

fn resolve_id(conn: &Connection, label_or_uuid: &str) -> String {
    if Uuid::parse_str(label_or_uuid).is_ok() {
        return label_or_uuid.to_string();
    }
    let map = id_map(conn);
    map.get(label_or_uuid)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| synthetic_uuid(label_or_uuid).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn confirm_persists_allocation_and_remainder() {
        let dir = tempdir().unwrap();
        let gate = PermissionGate::default();
        let wf = MatchingWorkflow {
            gate: &gate,
            paths: DataPaths::new(dir.path()),
            caller: "test".into(),
        };
        let snap = wf.snapshot().unwrap();
        assert_eq!(snap.transactions.len(), 2);
        let pay = wf.confirm("eco-synth-001", "doc-synth-001", None).unwrap();
        assert_eq!(pay.allocated_minor, 2499);
        let snap2 = wf.snapshot().unwrap();
        let tx = snap2
            .transactions
            .iter()
            .find(|t| t.id == "eco-synth-001")
            .unwrap();
        assert_eq!(tx.open_remainder_minor, 0);
        assert_eq!(tx.allocated_minor, 2499);
        let preview = wf
            .batch_preview(&[BatchItem {
                transaction_id: "eco-synth-001".into(),
                document_id: "doc-synth-001".into(),
            }])
            .unwrap();
        assert_eq!(preview.confirmable_count, 0);
        assert!(preview.exception_count >= 1);
    }

    #[test]
    fn reject_blocks_batch_confirm() {
        let dir = tempdir().unwrap();
        let gate = PermissionGate::default();
        let wf = MatchingWorkflow {
            gate: &gate,
            paths: DataPaths::new(dir.path()),
            caller: "test".into(),
        };
        wf.snapshot().unwrap();
        wf.reject("eco-synth-001", "doc-synth-001").unwrap();
        let preview = wf
            .batch_preview(&[BatchItem {
                transaction_id: "eco-synth-001".into(),
                document_id: "doc-synth-001".into(),
            }])
            .unwrap();
        assert_eq!(preview.confirmable_count, 0);
    }
}
