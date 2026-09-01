//! Exact installed Command Code headless integration for Swallowtail.
//!
//! The surface binds one qualified npm release and decodes its bounded NDJSON
//! `AgentEvent` stream. Structured runs keep `--no-session`. Interactive
//! continuity uses private same-cwd `--resume <sessionId>` without ambient
//! `--continue` / `--fork-session`, catalogue, export, TUI automation, or the
//! separate Provider API.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod activity;
mod command;
mod consumer_route_projection;
mod discovery;
mod driver;
mod events;
mod failure;
mod handle;
mod prepared;
mod pump;
mod selection;
mod session;
mod validation;

pub use access::{COMMAND_CODE_LOCAL_ACCOUNT_AUDIENCE, command_code_local_account_access_profile};
pub use driver::{CommandCodeHeadlessDriver, command_code_headless_descriptor};
pub use prepared::{
    CommandCodeHeadlessModelSelection, CommandCodePreparationInput, CommandCodePreparationProbe,
    CommandCodePreparedIntegration, CommandCodePreparedRun, CommandCodePreparedSession,
    CommandCodeRunProfileInput, CommandCodeSessionProfileInput, prepare_command_code_headless,
};
pub use selection::{
    COMMAND_CODE_EXECUTABLE_NAME, COMMAND_CODE_RELEASE_AXIS, COMMAND_CODE_RELEASE_VERSION,
    command_code_headless_claim, command_code_release_binding,
};

pub(crate) const DRIVER_ID: &str = "swallowtail.command-code.headless";
