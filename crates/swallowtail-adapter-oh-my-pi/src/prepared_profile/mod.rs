mod activity_profile;
mod catalogue;
mod input;
mod plan;
mod run;
mod session;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, RunHandle, RuntimeFailure};

pub use catalogue::OhMyPiPreparedCatalogue;
pub use input::{
    OhMyPiCatalogueProfileInput, OhMyPiModelSelection, OhMyPiRunProfileInput,
    OhMyPiSessionProfileInput,
};
pub use plan::OhMyPiPreparedEvidence;
pub use run::OhMyPiPreparedRun;
pub use session::OhMyPiPreparedSession;

/// Future returned when a prepared Oh My Pi session is opened.
pub type OhMyPiPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
/// Future returned when a prepared Oh My Pi structured run is started.
pub type OhMyPiPreparedRunFuture = BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>>;
