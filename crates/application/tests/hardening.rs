use belegwerk_application::actions::KnownAction;
use belegwerk_application::approval::ApprovalService;
use belegwerk_application::audit::InMemoryAuditSink;
use belegwerk_application::bank_import::{BankImportService, SourceTransactionInput};
use belegwerk_application::commands::{AppCommand, CommandBus};
use belegwerk_application::import_pipeline::{ImportFile, ImportPipeline};
use belegwerk_application::journal::{
    assert_balanced, can_post, draft_expense_against_bank, JournalError,
};
use belegwerk_application::matching_engine::{DocumentMatchInput, MatchingEngine};
use belegwerk_application::permission::{
    PermissionDecision, PermissionGate, PermissionRequest, ResourceRef, Scope,
};
use belegwerk_domain::accounting::{JournalLine, TaxRulePackRef};
use belegwerk_domain::{
    CalendarDate, CurrencyCode, DocumentId, EconomicTransaction, MatchConfidence, MoneyAmount,
    TransactionStatus,
};
use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use tempfile::tempdir;

#[test]
fn gate_denies_bank_pay_from_cli_and_mcp_callers() {
    let gate = PermissionGate::default();
    for caller in ["cli", "mcp", "plugin:evil", "desktop-ui"] {
        let req = PermissionRequest::for_known(
            KnownAction::BankPaymentCreate,
            ResourceRef {
                kind: "bank".into(),
                id: Some("acct".into()),
            },
            Scope {
                label: "test".into(),
            },
            "{}",
            caller,
        );
        assert!(
            matches!(gate.decide(&req), PermissionDecision::Deny { .. }),
            "caller {caller} must be denied"
        );
    }
}

#[test]
fn booking_post_requires_approval_even_when_capability_listed() {
    let gate = PermissionGate::new_with_grants([KnownAction::BookingPost]);
    let req = PermissionRequest::for_known(
        KnownAction::BookingPost,
        ResourceRef {
            kind: "journal".into(),
            id: Some("j1".into()),
        },
        Scope {
            label: "test".into(),
        },
        "payload",
        "ui",
    );
    assert_eq!(gate.decide(&req), PermissionDecision::RequireApproval);
}

#[test]
fn import_pipeline_is_idempotent_on_bytes_and_creates_two_meta_rows() {
    let dir = tempdir().unwrap();
    let paths = DataPaths::new(dir.path());
    let gate = PermissionGate::default();
    let pipeline = ImportPipeline {
        gate: &gate,
        paths: paths.clone(),
        caller: "test".into(),
    };
    let bytes = b"same-bytes-for-idempotency".to_vec();
    let file = ImportFile {
        filename: "a.txt".into(),
        mime_type: "text/plain".into(),
        bytes: bytes.clone(),
    };
    let a = pipeline.import_files("batch-a", &[file]).unwrap();
    let file2 = ImportFile {
        filename: "b.txt".into(),
        mime_type: "text/plain".into(),
        bytes,
    };
    let b = pipeline.import_files("batch-b", &[file2]).unwrap();
    assert_eq!(a.documents[0].content_hash, b.documents[0].content_hash);
    let conn = open_db(&paths.db_path()).unwrap();
    migrate(&conn).unwrap();
    let docs: i64 = conn
        .query_row("SELECT COUNT(*) FROM documents", [], |r| r.get(0))
        .unwrap();
    let batches: i64 = conn
        .query_row("SELECT COUNT(*) FROM import_batches", [], |r| r.get(0))
        .unwrap();
    assert_eq!(docs, 2);
    assert_eq!(batches, 2);
}

#[test]
fn matching_currency_mismatch_and_tolerance_edges() {
    let engine = MatchingEngine {
        amount_tolerance_minor: 5,
    };
    let eco = EconomicTransaction {
        id: Default::default(),
        amount: MoneyAmount::new(-1000, CurrencyCode::eur()),
        booking_date: CalendarDate::from_ymd(2026, 1, 1).unwrap(),
        description: Some("INV-1".into()),
        source_transaction_ids: vec![],
        status: TransactionStatus::Imported,
    };
    // currency mismatch alone never scores amount; unrelated ref → no hit
    let usd = vec![DocumentMatchInput {
        document_id: DocumentId::new(),
        amount: Some(MoneyAmount::new(1000, CurrencyCode("USD".into()))),
        invoice_ref: Some("OTHER-REF".into()),
        counterparty: None,
    }];
    assert!(engine.suggest_for_economic(&eco, &usd).is_empty());

    // within tolerance → amount match (medium) + ref → exact
    let near = vec![DocumentMatchInput {
        document_id: DocumentId::new(),
        amount: Some(MoneyAmount::new(1003, CurrencyCode::eur())),
        invoice_ref: Some("INV-1".into()),
        counterparty: None,
    }];
    let hits = engine.suggest_for_economic(&eco, &near);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].confidence, MatchConfidence::Exact);

    // outside tolerance, no ref → no hit
    let far = vec![DocumentMatchInput {
        document_id: DocumentId::new(),
        amount: Some(MoneyAmount::new(2000, CurrencyCode::eur())),
        invoice_ref: None,
        counterparty: None,
    }];
    assert!(engine.suggest_for_economic(&eco, &far).is_empty());
}

#[test]
fn journal_balance_and_draft_pack_gate() {
    let unbalanced = vec![
        JournalLine {
            account_code: "4950".into(),
            debit_minor: 10,
            credit_minor: 0,
            currency: "EUR".into(),
        },
        JournalLine {
            account_code: "1200".into(),
            debit_minor: 0,
            credit_minor: 9,
            currency: "EUR".into(),
        },
    ];
    assert!(matches!(
        assert_balanced(&unbalanced),
        Err(JournalError::Unbalanced {
            debit: 10,
            credit: 9
        })
    ));

    let draft_pack = TaxRulePackRef {
        id: "core".into(),
        version: "0".into(),
        status: "draft".into(),
        source_note: None,
    };
    assert_eq!(
        draft_expense_against_bank("j", "4950", "1200", 100, "EUR", draft_pack).unwrap_err(),
        JournalError::MissingRulePack
    );

    let reviewed = TaxRulePackRef {
        id: "core".into(),
        version: "0".into(),
        status: "reviewed".into(),
        source_note: Some("test".into()),
    };
    let ok = draft_expense_against_bank("j2", "4950", "1200", 100, "EUR", reviewed).unwrap();
    assert!(can_post(&ok).is_ok());
}

#[test]
fn bank_import_denied_without_read_capability_path_still_gates_unknown_action() {
    // DocumentsImport is allowed by default list; ensure CommandBus audits bank pay deny.
    let gate = PermissionGate::default();
    let approvals = ApprovalService::default();
    let audit = InMemoryAuditSink::default();
    let bus = CommandBus {
        gate: &gate,
        approvals: &approvals,
        audit: &audit,
        caller: "hardening".into(),
    };
    assert!(bus
        .dispatch(AppCommand::BankPaymentCreate {
            account_id: "x".into(),
            amount_minor: 1,
        })
        .is_err());
    assert!(audit
        .events()
        .iter()
        .any(|e| e.action == "bank.payment.create"));
}

#[test]
fn bank_import_service_rejects_bad_date() {
    let dir = tempdir().unwrap();
    let gate = PermissionGate::default();
    let svc = BankImportService {
        gate: &gate,
        paths: DataPaths::new(dir.path()),
        caller: "test".into(),
    };
    let bad = SourceTransactionInput {
        provider_id: "moneymoney".into(),
        source_account_id: "a".into(),
        source_transaction_ref: "bad".into(),
        booking_date: "not-a-date".into(),
        value_date: None,
        amount_minor: -1,
        currency: "EUR".into(),
        counterparty: None,
        purpose: None,
        raw_ref: None,
    };
    assert!(svc.import_many(&[bad]).is_err());
}
