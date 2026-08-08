mod activity_profile;
mod catalogue;
mod exec;
mod input;
mod management;
mod plan;
mod provider_session_import;
mod session;
mod session_capabilities;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, LoadedSession, RuntimeFailure};

pub use catalogue::CodexPreparedCatalogue;
pub use exec::CodexPreparedExec;
pub use input::{
    CodexExecProfileInput, CodexModelSelection, CodexSessionCatalogueInput,
    CodexSessionHistoryInput, CodexSessionManagementInput, CodexSessionProfileInput,
    CodexSessionReconciliationInput,
};
pub use management::{CodexPreparedArchive, CodexPreparedDelete, CodexPreparedRestore};
pub use plan::CodexPreparedEvidence;
pub use provider_session_import::{
    CodexPreparedSessionCatalogue, CodexPreparedSessionHistory, CodexPreparedSessionImport,
    CodexPreparedSessionReconciliation,
};
pub use session::CodexPreparedSession;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Access posture selected for a prepared Codex app-server session.
pub enum CodexPreparedSessionKind {
    /// Ambient harness session limited to read-only working-resource access.
    ReadOnly,
    /// Ambient harness session admitted to one bounded writable workspace.
    BoundedWorkspace,
}

/// Future returned when a prepared Codex session opens or resumes.
pub type CodexPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
/// Future returned when a prepared Codex session loads with replay.
pub type CodexPreparedSessionLoadFuture = BoxFuture<'static, Result<LoadedSession, RuntimeFailure>>;
