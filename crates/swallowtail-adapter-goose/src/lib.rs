//! Goose ACP harness driver.
//!
//! `goose.acp` binds host-approved `goose acp` for initialize plus one bounded
//! `session/prompt` through `prepare_goose_acp`. `goose serve`, `--with-builtin`,
//! `--enable-scheduler`, desktop, recipes as routing, and Goose ACP-providers stay out.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod acp_activity;
mod command;
mod connection;
mod consumer_route_projection;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod selection;
mod turn;

pub use access::{GOOSE_LOCAL_ACCOUNT_AUDIENCE, goose_local_config_access_profile};
pub use driver::{GooseAcpDriver, goose_acp_descriptor};
pub use prepared::{
    GoosePreparationInput, GoosePreparationProbe, GoosePreparedIntegration, GoosePreparedSession,
    GooseSessionProfileInput, prepare_goose_acp,
};
pub use selection::{
    GOOSE_EXECUTABLE_NAME, GOOSE_RELEASE_AXIS, GOOSE_RELEASE_VERSION, goose_acp_claim,
    goose_release_binding,
};
