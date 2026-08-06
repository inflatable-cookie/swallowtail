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

/// Future returned when a prepared Pi session is opened.
pub type PiPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
/// Future returned when a prepared Pi structured run is started.
pub type PiPreparedRunFuture = BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>>;
