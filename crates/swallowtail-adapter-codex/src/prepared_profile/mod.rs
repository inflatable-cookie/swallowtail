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
    CodexSessionManagementInput, CodexSessionProfileInput, CodexSessionReconciliationInput,
};
pub use management::{CodexPreparedArchive, CodexPreparedDelete, CodexPreparedRestore};
pub use plan::CodexPreparedEvidence;
pub use provider_session_import::{
    CodexPreparedSessionCatalogue, CodexPreparedSessionImport, CodexPreparedSessionReconciliation,
};
pub use session::CodexPreparedSession;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodexPreparedSessionKind {
    ReadOnly,
    BoundedWorkspace,
}

pub type CodexPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
pub type CodexPreparedSessionLoadFuture = BoxFuture<'static, Result<LoadedSession, RuntimeFailure>>;
