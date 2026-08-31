//! Deep Agents ACP harness driver.
//!
//! `deepagents.acp` binds host-approved `deepagents-acp` with no extra argv
//! for initialize plus one bounded `session/prompt` through
//! `prepare_deepagents_acp`. `npx`, library embed, `--workspace` / `--model`,
//! `session/load`, slash commands, and `session/prompt` field `content` stay
//! out. CLI `agentInfo.version` is the constructor default `0.0.1`, not npm
//! `0.1.25`.

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

pub use access::{
    DEEPAGENTS_PROVIDER_API_KEY_AUDIENCE, deepagents_provider_api_key_access_profile,
};
pub use driver::{DeepAgentsAcpDriver, deepagents_acp_descriptor};
pub use prepared::{
    DeepAgentsPreparationInput, DeepAgentsPreparationProbe, DeepAgentsPreparedIntegration,
    DeepAgentsPreparedSession, DeepAgentsSessionProfileInput, prepare_deepagents_acp,
};
pub use selection::{
    DEEPAGENTS_ACP_EXECUTABLE_NAME, DEEPAGENTS_ACP_PACKAGE_AXIS, DEEPAGENTS_ACP_PACKAGE_VERSION,
    deepagents_acp_claim, deepagents_acp_package_binding,
};
