//! GitHub Copilot CLI ACP harness driver.
//!
//! `copilot-cli.acp` binds host-approved `copilot --acp --stdio` for initialize
//! plus one bounded `session/prompt` through `prepare_copilot_cli_acp`. Public
//! preview stays visible. TCP `--port`, `--yolo`, `--allow-all`, server-start
//! tool/effort flags, interactive-only slash commands, and GitHub login stay out.

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

pub use access::{COPILOT_CLI_HOST_ACCOUNT_AUDIENCE, copilot_cli_host_account_access_profile};
pub use driver::{CopilotCliAcpDriver, copilot_cli_acp_descriptor};
pub use prepared::{
    CopilotCliPreparationInput, CopilotCliPreparationProbe, CopilotCliPreparedIntegration,
    CopilotCliPreparedSession, CopilotCliSessionProfileInput, prepare_copilot_cli_acp,
};
pub use selection::{
    COPILOT_CLI_ACP_MATURITY, COPILOT_CLI_EXECUTABLE_NAME, COPILOT_CLI_PACKAGE_AXIS,
    COPILOT_CLI_PACKAGE_VERSION, copilot_cli_acp_claim, copilot_cli_package_binding,
};
