//! OpenCode harness integrations for Swallowtail.
//!
//! `opencode.http` is the attached HTTP/SSE family. `opencode.acp` is a
//! separate ACP stdio family on `opencode.executable`. They are not flattened.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod acp;
mod activity;
mod consumer_route_projection;
mod driver;
mod failure;
mod prepared;
mod prepared_profile;
mod protocol;
mod selection;
mod transport;

pub use acp::{
    OPENCODE_ACP_AXIS, OPENCODE_ACP_BASELINE_VERSION, OPENCODE_ACP_EXECUTABLE_NAME,
    OPENCODE_ACP_HOST_ACCOUNT_AUDIENCE, OPENCODE_ACP_LATEST_QUALIFIED_VERSION,
    OPENCODE_ACP_MCP_SERVER_NAME, OpenCodeAcpDriver, OpenCodeAcpPreparationInput,
    OpenCodeAcpPreparationProbe, OpenCodeAcpPreparedIntegration, OpenCodeAcpPreparedSession,
    OpenCodeAcpRemoteMcpPlacement, OpenCodeAcpSessionProfileInput, OpenCodeAcpStdioMcpServer,
    opencode_acp_binding, opencode_acp_claim, opencode_acp_descriptor,
    opencode_acp_host_account_access_profile, prepare_opencode_acp,
};
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
