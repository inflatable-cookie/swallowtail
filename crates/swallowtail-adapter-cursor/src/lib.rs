//! Cursor Agent harness integration for Swallowtail.
//!
//! This foundation exposes exact installed discovery and the auth-aware local
//! model catalogue. ACP and headless execution remain separate later drivers.

#![forbid(unsafe_code)]

mod access;
mod catalogue;
mod descriptor;
mod discovery;
mod failure;
mod selection;

pub use access::{CURSOR_SUBSCRIPTION_AUDIENCE, cursor_subscription_access_profile};
pub use catalogue::CursorCatalogueDriver;
pub use descriptor::cursor_catalogue_descriptor;
pub use selection::{
    CURSOR_AGENT_AUTOMATIC_EXECUTABLE_NAME, CURSOR_AGENT_BASELINE_VERSION,
    CURSOR_AGENT_LATEST_QUALIFIED_BUILD_REVISION, CURSOR_AGENT_LATEST_QUALIFIED_VERSION,
    CURSOR_AGENT_RELEASE_AXIS, cursor_agent_release_binding, cursor_catalogue_claim,
};

const CATALOGUE_DRIVER_ID: &str = "swallowtail.cursor-agent.catalogue";
