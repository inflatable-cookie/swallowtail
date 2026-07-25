mod input;
mod operations;
mod plan;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, RuntimeFailure};

pub use input::{
    OpenCodeCatalogueProfileInput, OpenCodeModelSelection, OpenCodeSessionProfileInput,
};
pub use operations::{OpenCodePreparedCatalogue, OpenCodePreparedSession};
pub use plan::OpenCodePreparedEvidence;

pub type OpenCodePreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
