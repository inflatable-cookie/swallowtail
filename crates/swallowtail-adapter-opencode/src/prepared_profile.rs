mod input;
mod operations;
mod plan;

use swallowtail_runtime::{
    BoxFuture, InteractiveSessionHandle, LoadedSession, RunHandle, RuntimeFailure,
};

pub use input::{
    OpenCodeCatalogueProfileInput, OpenCodeModelSelection, OpenCodeRunProfileInput,
    OpenCodeSessionManagementInput, OpenCodeSessionProfileInput,
};
pub use operations::{
    OpenCodePreparedCatalogue, OpenCodePreparedDelete, OpenCodePreparedRun, OpenCodePreparedSession,
};
pub use plan::OpenCodePreparedEvidence;

pub type OpenCodePreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
pub type OpenCodePreparedSessionLoadFuture =
    BoxFuture<'static, Result<LoadedSession, RuntimeFailure>>;
pub type OpenCodePreparedRunFuture = BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>>;
