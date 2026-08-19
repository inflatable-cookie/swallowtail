//! Kiro ACP harness driver.
//!
//! `kiro.acp` binds host-approved `kiro-cli acp` for initialize plus one bounded
//! `session/prompt` through `prepare_kiro_acp`. `kiro-cli chat --no-interactive`,
//! `--cloud`, `--agent`, `--trust-all-tools`, `session/load`, and docs
//! `session/prompt` field `content` stay out.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod acp_activity;
mod command;
mod connection;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod selection;
mod turn;

pub use access::{KIRO_LOCAL_ACCOUNT_AUDIENCE, kiro_local_account_access_profile};
pub use driver::{KiroAcpDriver, kiro_acp_descriptor};
pub use prepared::{
    KiroPreparationInput, KiroPreparationProbe, KiroPreparedIntegration, KiroPreparedSession,
    KiroSessionProfileInput, prepare_kiro_acp,
};
pub use selection::{
    KIRO_CLI_EXECUTABLE_NAME, KIRO_CLI_RELEASE_AXIS, KIRO_CLI_RELEASE_VERSION, kiro_acp_claim,
    kiro_cli_release_binding,
};
