//! xAI direct-inference drivers for Swallowtail.
//!
//! The adapter owns a serial, resource-free Responses WebSocket session. It
//! does not enable provider storage, reconnect, retry, or durable resume.

#![forbid(unsafe_code)]

mod driver;
mod failure;
mod prepared;
mod prepared_profile;
mod protocol;
mod selection;
mod transport;

pub use driver::{XaiWebSocketDriver, xai_websocket_descriptor};
pub use prepared::{XaiPreparationInput, XaiPreparedIntegration, prepare_xai_responses_websocket};
pub use prepared_profile::{
    XaiModelSelection, XaiPreparedEvidence, XaiPreparedResponsesSession, XaiSessionProfileInput,
};
pub use selection::{
    XAI_RESPONSES_ACCESS_PROFILE_ID, XAI_RESPONSES_CONFIGURED_INSTANCE_ID, XAI_RESPONSES_ENDPOINT,
    XAI_RESPONSES_ENDPOINT_AUDIENCE, XAI_RESPONSES_FACADE_REVISION, xai_responses_access_profile,
    xai_responses_facade_binding, xai_responses_facade_claim, xai_responses_instance,
    xai_responses_model_route, xai_responses_requirements,
};

/// Provider-supported Responses WebSocket route frozen by the first fixture.
pub const RESPONSES_WEBSOCKET_PATH: &str = "/v1/responses";

/// Exact scale used by xAI's provider-authored billed-cost observation.
pub const USD_TICKS_PER_USD: u64 = 10_000_000_000;
