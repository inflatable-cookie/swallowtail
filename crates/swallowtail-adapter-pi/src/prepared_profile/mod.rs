mod activity_profile;
mod catalogue;
mod input;
mod plan;
mod run;
mod session;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, RunHandle, RuntimeFailure};

pub use catalogue::PiPreparedCatalogue;
pub use input::{
    PiCatalogueProfileInput, PiModelSelection, PiRunProfileInput, PiSessionProfileInput,
};
pub use plan::PiPreparedEvidence;
pub use run::PiPreparedRun;
pub use session::PiPreparedSession;

pub type PiPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
pub type PiPreparedRunFuture = BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>>;
