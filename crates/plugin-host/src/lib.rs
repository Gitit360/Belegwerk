//! Plugin host stub (ADR-0003). In-process for unit tests; real IPC in M02+.

pub mod manifest;
pub mod protocol;
pub mod runtime;

pub use manifest::{PluginKind, PluginManifest, PluginPermissions, SdkVersion};
pub use protocol::{MessageType, PluginMessage};
pub use runtime::{HostError, PluginRuntime};
