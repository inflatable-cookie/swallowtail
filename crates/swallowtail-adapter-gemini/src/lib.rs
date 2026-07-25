//! Gemini CLI ACP integration for Swallowtail.

#![forbid(unsafe_code)]

mod connection;
mod discovery;
mod driver;
mod failure;
mod live;
mod live_protocol;
mod live_selection;
mod prepared;
mod prepared_live;
mod prepared_live_profile;
mod prepared_profile;
mod selection;
mod turn;

pub use driver::{GeminiAcpDriver, gemini_acp_descriptor};
pub use live::{GeminiLiveDriver, gemini_live_descriptor};
pub use live_selection::{
    GEMINI_LIVE_ACCESS_PROFILE_ID, GEMINI_LIVE_CONFIGURED_INSTANCE_ID, GEMINI_LIVE_ENDPOINT,
    GEMINI_LIVE_ENDPOINT_AUDIENCE, GEMINI_LIVE_FACADE_REVISION, GEMINI_LIVE_MODEL_ID,
    GEMINI_LIVE_MODEL_ROUTE_ID, gemini_live_access_profile, gemini_live_facade_binding,
    gemini_live_facade_claim, gemini_live_instance, gemini_live_media_config,
    gemini_live_model_route, gemini_live_requirements, gemini_live_rollover_policy,
};
pub use prepared::{
    GeminiPreparationInput, GeminiPreparationProbe, GeminiPreparedIntegration, prepare_gemini_acp,
};
pub use prepared_live::{
    GeminiLivePreparationInput, GeminiLivePreparedIntegration, prepare_gemini_live,
};
pub use prepared_live_profile::{
    GeminiLivePreparedEvidence, GeminiLiveSessionProfileInput, GeminiPreparedLiveSession,
};
pub use prepared_profile::{
    GeminiPreparedEvidence, GeminiPreparedSession, GeminiPreparedSessionFuture,
    GeminiSessionProfileInput,
};
pub use selection::{
    GEMINI_CLI_ACP_AXIS, GEMINI_CLI_ACP_BASELINE_VERSION, GEMINI_CLI_ACP_LATEST_QUALIFIED_VERSION,
    gemini_cli_acp_binding, gemini_cli_acp_claim,
};
