mod catalogue;
mod input;
mod plan;
mod session;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, RuntimeFailure};

pub use catalogue::PiPreparedCatalogue;
pub use input::{PiCatalogueProfileInput, PiModelSelection, PiSessionProfileInput};
pub use plan::PiPreparedEvidence;
pub use session::PiPreparedSession;

pub type PiPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
