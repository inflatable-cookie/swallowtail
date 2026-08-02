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

pub type KimiPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
pub type KimiPreparedSessionLoadFuture = BoxFuture<'static, Result<LoadedSession, RuntimeFailure>>;
