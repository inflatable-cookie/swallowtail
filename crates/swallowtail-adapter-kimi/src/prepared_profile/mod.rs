mod activity_profile;
mod import;
mod input;
mod plan;
mod provider_session_catalogue;
mod session;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, LoadedSession, RuntimeFailure};

pub use import::KimiAcpSessionImportAuthority;
pub use input::{KimiModelSelection, KimiSessionCatalogueInput, KimiSessionProfileInput};
pub use plan::KimiPreparedEvidence;
pub use provider_session_catalogue::{KimiPreparedSessionCatalogue, KimiPreparedSessionImport};
pub use session::KimiPreparedSession;

/// Future returned when a prepared Kimi ACP session opens or resumes.
pub type KimiPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
/// Future returned when a prepared Kimi ACP session loads with replay.
pub type KimiPreparedSessionLoadFuture = BoxFuture<'static, Result<LoadedSession, RuntimeFailure>>;
