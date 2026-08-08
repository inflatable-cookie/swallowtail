//! Attached OpenCode HTTP harness integration for Swallowtail.
//!
//! The adapter keeps model catalogue, interactive and structured execution,
//! provider-session catalogue/import/reconciliation, and inactive-session
//! deletion as separately prepared authorities against one observed server.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod activity;
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
    OpenCodePreparedDelete, OpenCodePreparedEvidence, OpenCodePreparedRun,
    OpenCodePreparedRunFuture, OpenCodePreparedSession, OpenCodePreparedSessionCatalogue,
    OpenCodePreparedSessionFuture, OpenCodePreparedSessionHistory, OpenCodePreparedSessionImport,
    OpenCodePreparedSessionLoadFuture, OpenCodePreparedSessionReconciliation,
    OpenCodeRunProfileInput, OpenCodeSessionCatalogueInput, OpenCodeSessionHistoryInput,
    OpenCodeSessionManagementInput, OpenCodeSessionProfileInput,
    OpenCodeSessionReconciliationInput,
};
pub use selection::{
    OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION, OPENCODE_SERVER_AXIS,
    opencode_http_claim, opencode_server_binding,
};
