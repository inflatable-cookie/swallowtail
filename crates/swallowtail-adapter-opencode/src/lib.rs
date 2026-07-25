//! Attached OpenCode HTTP harness integration for Swallowtail.

#![forbid(unsafe_code)]

mod driver;
mod failure;
mod prepared;
mod prepared_profile;
mod protocol;
mod selection;
mod transport;

pub use driver::{OpenCodeHttpDriver, opencode_http_descriptor};
pub use prepared::{
    OpenCodePreparationInput, OpenCodePreparationProbe, OpenCodePreparedIntegration,
    OpenCodePreparedServerObservation, prepare_opencode_attached,
};
pub use prepared_profile::{
    OpenCodeCatalogueProfileInput, OpenCodeModelSelection, OpenCodePreparedCatalogue,
    OpenCodePreparedEvidence, OpenCodePreparedSession, OpenCodePreparedSessionFuture,
    OpenCodeSessionProfileInput,
};
pub use selection::{
    OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION, OPENCODE_SERVER_AXIS,
    opencode_http_claim, opencode_server_binding,
};
