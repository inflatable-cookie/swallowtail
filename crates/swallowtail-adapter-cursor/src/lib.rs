//! Cursor Agent harness integration for Swallowtail.
//!
//! Exact installed discovery, auth-aware model catalogue, and first-party ACP
//! interactive sessions remain separate drivers.

#![forbid(unsafe_code)]

mod access;
mod activity;
mod catalogue;
mod connection;
mod descriptor;
mod discovery;
mod driver;
mod failure;
mod headless;
mod headless_activity;
mod headless_command;
mod headless_events;
mod headless_handle;
mod headless_pump;
mod headless_validation;
mod prepared;
mod selection;
mod turn;

pub use access::{CURSOR_SUBSCRIPTION_AUDIENCE, cursor_subscription_access_profile};
pub use catalogue::CursorCatalogueDriver;
pub use descriptor::{
    cursor_acp_descriptor, cursor_catalogue_descriptor, cursor_headless_descriptor,
};
pub use discovery::CursorAcpDriver;
pub use headless::CursorHeadlessDriver;
pub use prepared::{
    CursorAcpSessionProfileInput, CursorCatalogueProfileInput, CursorHeadlessModelSelection,
    CursorHeadlessRunProfileInput, CursorPreparationInput, CursorPreparationProbe,
    CursorPreparedAcpIntegration, CursorPreparedAcpSession, CursorPreparedCatalogue,
    CursorPreparedCatalogueIntegration, CursorPreparedDriver, CursorPreparedHeadlessIntegration,
    CursorPreparedHeadlessRun, CursorPreparedIntegration, prepare_cursor,
};
pub use selection::{
    CURSOR_AGENT_AUTOMATIC_EXECUTABLE_NAME, CURSOR_AGENT_BASELINE_BUILD_REVISION,
    CURSOR_AGENT_BASELINE_VERSION, CURSOR_AGENT_LATEST_QUALIFIED_BUILD_REVISION,
    CURSOR_AGENT_LATEST_QUALIFIED_VERSION, CURSOR_AGENT_RELEASE_AXIS, cursor_acp_claim,
    cursor_agent_release_binding, cursor_catalogue_claim, cursor_headless_claim,
};

const CATALOGUE_DRIVER_ID: &str = "swallowtail.cursor-agent.catalogue";
const ACP_DRIVER_ID: &str = "swallowtail.cursor-agent.acp";
const HEADLESS_DRIVER_ID: &str = "swallowtail.cursor-agent.headless";
