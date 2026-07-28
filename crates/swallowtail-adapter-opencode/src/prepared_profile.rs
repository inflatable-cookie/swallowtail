mod input;
mod operations;
mod plan;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, RunHandle, RuntimeFailure};

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
pub type OpenCodePreparedRunFuture = BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>>;
