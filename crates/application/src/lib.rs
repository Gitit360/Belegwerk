//! Application layer — permission gate cannot be bypassed by UI/plugins.

pub mod actions;
pub mod approval;
pub mod archive;
pub mod audit;
pub mod commands;
pub mod import_pipeline;
pub mod journal;
pub mod local_api;
pub mod matching_engine;
pub mod matching_service;
pub mod permission;

pub use approval::{ActionApproval, ApprovalService};
pub use audit::{AuditEvent, AuditSink, InMemoryAuditSink};
pub use commands::{AppCommand, CommandBus, CommandError, CommandResult};
pub use permission::{
    ActionId, PermissionDecision, PermissionGate, PermissionRequest, ResourceRef, Scope,
};
