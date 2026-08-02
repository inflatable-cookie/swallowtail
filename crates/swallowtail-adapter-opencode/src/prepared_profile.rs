mod input;
mod operations;
mod plan;
mod provider_sessions;

use swallowtail_runtime::{
    BoxFuture, InteractiveSessionHandle, LoadedSession, RunHandle, RuntimeFailure,
};

pub use input::{
    OpenCodeCatalogueProfileInput, OpenCodeModelSelection, OpenCodeRunProfileInput,
    OpenCodeSessionCatalogueInput, OpenCodeSessionManagementInput, OpenCodeSessionProfileInput,
};
pub use operations::{
    OpenCodePreparedCatalogue, OpenCodePreparedDelete, OpenCodePreparedRun, OpenCodePreparedSession,
};
pub use plan::OpenCodePreparedEvidence;
pub use provider_sessions::{OpenCodePreparedSessionCatalogue, OpenCodePreparedSessionImport};

pub type OpenCodePreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
pub type OpenCodePreparedSessionLoadFuture =
    BoxFuture<'static, Result<LoadedSession, RuntimeFailure>>;
pub type OpenCodePreparedRunFuture = BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>>;
