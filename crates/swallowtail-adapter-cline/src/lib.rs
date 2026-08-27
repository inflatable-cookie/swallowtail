//! Cline ACP and headless harness drivers.
//!
//! `cline.acp` binds host-approved `cline --acp` for initialize plus one bounded
//! `session/prompt`, with optional portable `HarnessMode::Plan` negotiated
//! through `session/set_config_option` before the first prompt.
//! `cline.headless` binds `cline --json --auto-approve false` for one bounded
//! print run, with optional portable `HarnessMode::Plan` as canonical `--plan`.
//! Hub/TUI, `--id`, and `--auto-approve true` stay out of both routes.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod acp_activity;
mod command;
mod connection;
mod discovery;
mod driver;
mod failure;
mod headless;
mod prepared;
mod prepared_headless;
mod selection;
mod turn;

pub use access::{CLINE_LOCAL_ACCOUNT_AUDIENCE, cline_local_account_access_profile};
pub use driver::{ClineAcpDriver, cline_acp_descriptor};
pub use headless::{ClineHeadlessDriver, cline_headless_descriptor};
pub use prepared::{
    ClinePreparationInput, ClinePreparationProbe, ClinePreparedIntegration, ClinePreparedSession,
    ClineSessionProfileInput, prepare_cline_acp,
};
pub use prepared_headless::{
    ClineHeadlessPreparationInput, ClineHeadlessPreparationProbe, ClineHeadlessPreparedIntegration,
    ClineHeadlessPreparedRun, ClineHeadlessRunProfileInput, prepare_cline_headless,
};
pub use selection::{
    CLINE_EXECUTABLE_NAME, CLINE_PACKAGE_AXIS, CLINE_PACKAGE_VERSION, cline_acp_claim,
    cline_headless_claim, cline_package_binding,
};
