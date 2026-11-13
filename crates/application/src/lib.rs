//! Application layer — permission gate cannot be bypassed by UI/plugins.

pub mod actions;
pub mod ai_router;
pub mod approval;
pub mod archive;
pub mod audit;
pub mod bank_import;
pub mod commands;
pub mod extensions;
pub mod extraction_service;
pub mod grandtotal_import;
pub mod http_stub;
pub mod import_pipeline;
pub mod import_query;
pub mod jobs;
pub mod journal;
pub mod journal_store;
pub mod local_api;
pub mod mail_portal;
pub mod matching_engine;
pub mod matching_service;
pub mod matching_workflow;
pub mod moneymoney_replay;
pub mod permission;
pub mod reminders_service;
pub mod reports;
pub mod settings_service;

pub use approval::{ActionApproval, ApprovalService};
pub use audit::{AuditEvent, AuditSink, InMemoryAuditSink};
pub use commands::{AppCommand, CommandBus, CommandError, CommandResult};
pub use permission::{
    ActionId, PermissionDecision, PermissionGate, PermissionRequest, ResourceRef, Scope,
};
