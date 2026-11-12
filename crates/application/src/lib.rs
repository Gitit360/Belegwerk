//! Application layer — permission gate cannot be bypassed by UI/plugins.

pub mod actions;
pub mod approval;
pub mod archive;
pub mod audit;
pub mod local_api;
pub mod matching_service;
pub mod permission;

pub use approval::{ActionApproval, ApprovalService};
pub use audit::{AuditEvent, AuditSink, InMemoryAuditSink};
pub use permission::{
    ActionId, PermissionDecision, PermissionGate, PermissionRequest, ResourceRef, Scope,
};
