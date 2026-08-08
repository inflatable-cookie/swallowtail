mod input;
mod operations;
mod plan;
mod provider_sessions;

use swallowtail_runtime::{
    BoxFuture, InteractiveSessionHandle, LoadedSession, RunHandle, RuntimeFailure,
};

pub use input::{
    OpenCodeCatalogueProfileInput, OpenCodeModelSelection, OpenCodeRunProfileInput,
    OpenCodeSessionCatalogueInput, OpenCodeSessionHistoryInput, OpenCodeSessionManagementInput,
    OpenCodeSessionProfileInput, OpenCodeSessionReconciliationInput,
};
pub use operations::{
    OpenCodePreparedCatalogue, OpenCodePreparedDelete, OpenCodePreparedRun, OpenCodePreparedSession,
};
pub use plan::OpenCodePreparedEvidence;
pub use provider_sessions::{
    OpenCodePreparedSessionCatalogue, OpenCodePreparedSessionHistory,
    OpenCodePreparedSessionImport, OpenCodePreparedSessionReconciliation,
};

/// Future returned when a prepared OpenCode session is opened or resumed.
pub type OpenCodePreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
/// Future returned when a prepared retained session is loaded with replay.
pub type OpenCodePreparedSessionLoadFuture =
    BoxFuture<'static, Result<LoadedSession, RuntimeFailure>>;
/// Future returned when a prepared structured run is started.
pub type OpenCodePreparedRunFuture = BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>>;
