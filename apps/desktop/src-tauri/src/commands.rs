//! Typed invoke commands for core workflows (browser fallbacks live in the React shell).

use belegwerk_application::accounts_workflow::AccountsWorkflow;
use belegwerk_application::approval::ApprovalService;
use belegwerk_application::archive_workflow::ArchiveWorkflow;
use belegwerk_application::audit::InMemoryAuditSink;
use belegwerk_application::import_pipeline::{ImportFile, ImportPipeline};
use belegwerk_application::import_query::ImportQuery;
use belegwerk_application::journal_store::JournalStore;
use belegwerk_application::macos_probes;
use belegwerk_application::matching_workflow::{BatchItem, MatchingWorkflow};
use belegwerk_application::permission::PermissionGate;
use belegwerk_application::reminders_service::ReminderService;
use belegwerk_application::reports_workflow::ReportsWorkflow;
use belegwerk_application::settings_service::SettingsService;
use belegwerk_domain::settings::TaxProfileSettings;
use belegwerk_infrastructure::{migrate, open_db, DataPaths};
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

pub struct AppState {
    pub data_root: Mutex<Option<PathBuf>>,
    pub gate: PermissionGate,
    pub approvals: Mutex<ApprovalService>,
    pub audit: InMemoryAuditSink,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            data_root: Mutex::new(None),
            gate: PermissionGate::default(),
            approvals: Mutex::new(ApprovalService::default()),
            audit: InMemoryAuditSink::default(),
        }
    }
}

fn paths(state: &AppState) -> Result<DataPaths, String> {
    let root = state
        .data_root
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "data root not initialized — call init_data_root first".to_string())?;
    Ok(DataPaths::new(root))
}

#[tauri::command]
pub fn matching_snapshot(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let paths = paths(&state)?;
    let wf = MatchingWorkflow {
        gate: &state.gate,
        paths,
        caller: "desktop-ui".into(),
    };
    let snap = wf.snapshot().map_err(|e| e.to_string())?;
    serde_json::to_value(snap).map_err(|e| e.to_string())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchPairArgs {
    pub transaction_id: String,
    pub document_id: String,
    pub allocated_minor: Option<i64>,
    pub private_percent: Option<u8>,
}

#[tauri::command]
pub fn matching_confirm(
    state: State<'_, AppState>,
    args: MatchPairArgs,
) -> Result<serde_json::Value, String> {
    let paths = paths(&state)?;
    let wf = MatchingWorkflow {
        gate: &state.gate,
        paths,
        caller: "desktop-ui".into(),
    };
    let private = args.private_percent.unwrap_or(0);
    let pay = wf
        .confirm_with_private_share(
            &args.transaction_id,
            &args.document_id,
            args.allocated_minor,
            private,
        )
        .map_err(|e| e.to_string())?;
    serde_json::to_value(pay).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn matching_rebuild(state: State<'_, AppState>) -> Result<usize, String> {
    let paths = paths(&state)?;
    let wf = MatchingWorkflow {
        gate: &state.gate,
        paths,
        caller: "desktop-ui".into(),
    };
    wf.rebuild_suggestions().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn matching_reject(
    state: State<'_, AppState>,
    args: MatchPairArgs,
) -> Result<serde_json::Value, String> {
    let paths = paths(&state)?;
    let wf = MatchingWorkflow {
        gate: &state.gate,
        paths,
        caller: "desktop-ui".into(),
    };
    let d = wf
        .reject(&args.transaction_id, &args.document_id)
        .map_err(|e| e.to_string())?;
    serde_json::to_value(d).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn matching_batch_preview(
    state: State<'_, AppState>,
    items: Vec<BatchItem>,
) -> Result<serde_json::Value, String> {
    let paths = paths(&state)?;
    let wf = MatchingWorkflow {
        gate: &state.gate,
        paths,
        caller: "desktop-ui".into(),
    };
    let preview = wf.batch_preview(&items).map_err(|e| e.to_string())?;
    serde_json::to_value(preview).map_err(|e| e.to_string())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportFileArgs {
    pub filename: String,
    pub mime_type: String,
    pub text_content: String,
}

#[tauri::command]
pub fn import_synthetic_files(
    state: State<'_, AppState>,
    source_label: String,
    files: Vec<ImportFileArgs>,
) -> Result<serde_json::Value, String> {
    let data_paths = paths(&state)?;
    let pipeline = ImportPipeline {
        gate: &state.gate,
        paths: data_paths,
        caller: "desktop-ui".into(),
    };
    let mapped: Vec<ImportFile> = files
        .into_iter()
        .map(|f| ImportFile {
            filename: f.filename,
            mime_type: f.mime_type,
            bytes: f.text_content.into_bytes(),
        })
        .collect();
    let out = pipeline
        .import_files(source_label, &mapped)
        .map_err(|e| e.to_string())?;
    let data_paths = paths(&state)?;
    let rebuilt = MatchingWorkflow {
        gate: &state.gate,
        paths: data_paths,
        caller: "desktop-ui".into(),
    }
    .rebuild_suggestions()
    .unwrap_or(0);
    Ok(serde_json::json!({
        "batchId": out.batch.id.to_string(),
        "documentCount": out.documents.len(),
        "hashes": out.documents.iter().map(|d| d.content_hash.as_str().to_string()).collect::<Vec<_>>(),
        "matchCandidates": rebuilt,
    }))
}

#[tauri::command]
pub fn import_preview_blob(
    state: State<'_, AppState>,
    content_hash: String,
) -> Result<String, String> {
    let arch = ArchiveWorkflow {
        paths: paths(&state)?,
    };
    arch.preview_text(&content_hash).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn accounts_snapshot(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let wf = AccountsWorkflow {
        gate: &state.gate,
        paths: paths(&state)?,
        caller: "desktop-ui".into(),
    };
    serde_json::to_value(wf.snapshot().map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn accounts_replay_fixtures(
    state: State<'_, AppState>,
    dir: Option<String>,
) -> Result<usize, String> {
    let wf = AccountsWorkflow {
        gate: &state.gate,
        paths: paths(&state)?,
        caller: "desktop-ui".into(),
    };
    let n = if let Some(d) = dir {
        wf.replay_bank_fixtures(std::path::Path::new(&d))
            .map_err(|e| e.to_string())?
    } else {
        wf.replay_default_synthetic_bank()
            .map_err(|e| e.to_string())?
    };
    let _ = MatchingWorkflow {
        gate: &state.gate,
        paths: paths(&state)?,
        caller: "desktop-ui".into(),
    }
    .rebuild_suggestions();
    Ok(n)
}

#[tauri::command]
pub fn accounts_live_moneymoney(
    state: State<'_, AppState>,
    from_date: Option<String>,
) -> Result<usize, String> {
    let wf = AccountsWorkflow {
        gate: &state.gate,
        paths: paths(&state)?,
        caller: "desktop-ui".into(),
    };
    let n = wf
        .import_live_moneymoney(from_date.as_deref())
        .map_err(|e| e.to_string())?;
    let _ = MatchingWorkflow {
        gate: &state.gate,
        paths: paths(&state)?,
        caller: "desktop-ui".into(),
    }
    .rebuild_suggestions();
    Ok(n)
}

#[tauri::command]
pub fn mail_live_search(
    subject_contains: String,
    limit: Option<usize>,
) -> Result<serde_json::Value, String> {
    let hits = belegwerk_application::mail_live::search_inbox_subject(
        &subject_contains,
        limit.unwrap_or(20),
    )
    .map_err(|e| e.to_string())?;
    serde_json::to_value(hits).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn keychain_ensure_slot() -> Result<String, String> {
    belegwerk_application::mail_live::ensure_keychain_slot().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reports_snapshot(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let wf = ReportsWorkflow {
        paths: paths(&state)?,
    };
    serde_json::to_value(wf.snapshot().map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn archive_snapshot(
    state: State<'_, AppState>,
    query: Option<String>,
) -> Result<serde_json::Value, String> {
    let wf = ArchiveWorkflow {
        paths: paths(&state)?,
    };
    serde_json::to_value(
        wf.snapshot(query.as_deref())
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn macos_capability_probes() -> serde_json::Value {
    serde_json::to_value(macos_probes::run_probes()).unwrap_or_else(|_| serde_json::json!({}))
}

#[tauri::command]
pub fn import_list_batches(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let q = ImportQuery {
        paths: paths(&state)?,
    };
    serde_json::to_value(q.list_batches().map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_list_documents(
    state: State<'_, AppState>,
    batch_id: Option<String>,
) -> Result<serde_json::Value, String> {
    let q = ImportQuery {
        paths: paths(&state)?,
    };
    serde_json::to_value(
        q.list_documents(batch_id.as_deref())
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reminders_list(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let svc = ReminderService {
        paths: paths(&state)?,
    };
    svc.seed_demo_if_empty().map_err(|e| e.to_string())?;
    serde_json::to_value(svc.list_open_detailed().map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reminders_escalate(state: State<'_, AppState>, case_id: String) -> Result<u8, String> {
    authorize_simple(&state, belegwerk_application::actions::KnownAction::ReminderEscalate)?;
    let svc = ReminderService {
        paths: paths(&state)?,
    };
    let id = Uuid::parse_str(&case_id).map_err(|e| e.to_string())?;
    svc.escalate(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reminders_complete(state: State<'_, AppState>, case_id: String) -> Result<(), String> {
    authorize_simple(&state, belegwerk_application::actions::KnownAction::ReminderComplete)?;
    let svc = ReminderService {
        paths: paths(&state)?,
    };
    let id = Uuid::parse_str(&case_id).map_err(|e| e.to_string())?;
    svc.complete(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn journal_list_drafts(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let store = JournalStore {
        gate: &state.gate,
        paths: paths(&state)?,
        caller: "desktop-ui".into(),
    };
    serde_json::to_value(store.list_drafts().map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn journal_approve_draft(state: State<'_, AppState>, draft_id: String) -> Result<String, String> {
    let store = JournalStore {
        gate: &state.gate,
        paths: paths(&state)?,
        caller: "desktop-ui".into(),
    };
    let approvals = state.approvals.lock().map_err(|e| e.to_string())?;
    store
        .approve_draft(&draft_id, &approvals, &state.audit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn journal_grant_booking_approval(
    state: State<'_, AppState>,
    draft_id: String,
) -> Result<(), String> {
    // Explicit local approval grant for desktop demo — still audited via subsequent BookingPost.
    use belegwerk_application::actions::KnownAction;
    use belegwerk_application::commands::AppCommand;
    use belegwerk_application::permission::{ActionId, ResourceRef};
    use chrono::{Duration, Utc};
    use sha2::{Digest, Sha256};

    let cmd = AppCommand::BookingPost {
        journal_id: draft_id.clone(),
    };
    let mut hasher = Sha256::new();
    hasher.update(cmd.payload_for_hash().as_bytes());
    let payload_hash = hex::encode(hasher.finalize());
    let mut approvals = state.approvals.lock().map_err(|e| e.to_string())?;
    approvals.approve(
        ActionId(KnownAction::BookingPost.as_str().into()),
        ResourceRef {
            kind: "journal".into(),
            id: Some(draft_id),
        },
        payload_hash,
        "desktop-ui",
        Some(Utc::now() + Duration::hours(1)),
    );
    Ok(())
}

#[tauri::command]
pub fn settings_get_tax_profile(state: State<'_, AppState>) -> Result<TaxProfileSettings, String> {
    let svc = SettingsService {
        gate: &state.gate,
        paths: paths(&state)?,
        caller: "desktop-ui".into(),
    };
    svc.get_tax_profile().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn settings_save_tax_profile(
    state: State<'_, AppState>,
    profile: TaxProfileSettings,
) -> Result<TaxProfileSettings, String> {
    let svc = SettingsService {
        gate: &state.gate,
        paths: paths(&state)?,
        caller: "desktop-ui".into(),
    };
    svc.save_tax_profile(&profile).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ensure_schema(state: State<'_, AppState>) -> Result<i64, String> {
    let paths = paths(&state)?;
    paths.ensure().map_err(|e| e.to_string())?;
    let conn = open_db(&paths.db_path()).map_err(|e| e.to_string())?;
    migrate(&conn).map_err(|e| e.to_string())
}

fn authorize_simple(
    state: &AppState,
    action: belegwerk_application::actions::KnownAction,
) -> Result<(), String> {
    use belegwerk_application::permission::{PermissionRequest, ResourceRef, Scope};
    let req = PermissionRequest::for_known(
        action,
        ResourceRef {
            kind: "reminder".into(),
            id: None,
        },
        Scope {
            label: "desktop".into(),
        },
        "{}",
        "desktop-ui",
    );
    state.gate.authorize(&req).map_err(|e| e.to_string())
}
