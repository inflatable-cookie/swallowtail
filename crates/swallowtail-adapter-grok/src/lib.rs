//! Grok Build ACP integration for Swallowtail.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod activity;
mod connection;
mod descriptor;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod prepared_profile;
mod selection;
mod turn;

pub use descriptor::grok_build_acp_descriptor;
pub use discovery::GrokAcpDriver;
pub use prepared::{
    GrokPreparationInput, GrokPreparationProbe, GrokPreparedIntegration, prepare_grok_build,
};
pub use prepared_profile::{
    GrokModelSelection, GrokPreparedEvidence, GrokPreparedRun, GrokPreparedRunFuture,
    GrokPreparedSession, GrokPreparedSessionFuture, GrokRunProfileInput, GrokSessionProfileInput,
};
pub use selection::{
    GROK_BUILD_ACP_AXIS, GROK_BUILD_ACP_BASELINE_VERSION, GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION,
    GROK_BUILD_SUBSCRIPTION_ACCESS_PROFILE_ID, GROK_BUILD_SUBSCRIPTION_AUDIENCE,
    grok_build_acp_binding, grok_build_acp_claim, grok_build_model_for_behavior,
    grok_build_model_for_version, grok_build_subscription_access_profile,
};

const MAXIMUM_ATTACHMENT_RECOVERY_UPDATES: usize = 4096;
const MAXIMUM_ATTACHMENT_RECOVERY_BYTES: usize = 8 * 1024 * 1024;
