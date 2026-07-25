//! OpenAI public-API drivers for Swallowtail.
//!
//! Provider-managed Responses background runs and Realtime media connections
//! remain separate production drivers and prepared-profile lanes.

#![forbid(unsafe_code)]

mod driver;
mod failure;
mod prepared;
mod prepared_profile;
mod prepared_realtime;
mod prepared_realtime_profile;
mod protocol;
mod realtime;
mod realtime_protocol;
mod realtime_selection;
mod selection;
mod transport;

pub use driver::{OpenAiBackgroundDriver, openai_background_descriptor};
pub use prepared::{
    OpenAiBackgroundPreparationInput, OpenAiBackgroundPreparedIntegration,
    prepare_openai_background,
};
pub use prepared_profile::{
    OpenAiBackgroundModelSelection, OpenAiBackgroundPreparedEvidence,
    OpenAiBackgroundRunProfileInput, OpenAiPreparedBackgroundRun,
};
pub use prepared_realtime::{
    OpenAiRealtimePreparationInput, OpenAiRealtimePreparedIntegration, prepare_openai_realtime,
};
pub use prepared_realtime_profile::{
    OpenAiPreparedRealtimeSession, OpenAiRealtimePreparedEvidence,
    OpenAiRealtimeSessionProfileInput,
};
pub use realtime::{OpenAiRealtimeDriver, openai_realtime_descriptor};
pub use realtime_selection::{
    OPENAI_REALTIME_ACCESS_PROFILE_ID, OPENAI_REALTIME_CONFIGURED_INSTANCE_ID,
    OPENAI_REALTIME_ENDPOINT, OPENAI_REALTIME_ENDPOINT_AUDIENCE, OPENAI_REALTIME_FACADE_REVISION,
    OPENAI_REALTIME_MODEL_ID, OPENAI_REALTIME_MODEL_ROUTE_ID, openai_realtime_access_profile,
    openai_realtime_facade_binding, openai_realtime_facade_claim, openai_realtime_instance,
    openai_realtime_media_config, openai_realtime_model_route, openai_realtime_requirements,
};
pub use selection::{
    OPENAI_BACKGROUND_ACCESS_PROFILE_ID, OPENAI_BACKGROUND_CONFIGURED_INSTANCE_ID,
    OPENAI_BACKGROUND_ENDPOINT, OPENAI_BACKGROUND_ENDPOINT_AUDIENCE,
    OPENAI_BACKGROUND_FACADE_REVISION, OPENAI_BACKGROUND_MODEL_ID,
    OPENAI_BACKGROUND_MODEL_ROUTE_ID, openai_background_access_profile,
    openai_background_facade_binding, openai_background_facade_claim, openai_background_instance,
    openai_background_model_route, openai_background_requirements,
};

pub(crate) const ENDPOINT_AUDIENCE: &str = OPENAI_BACKGROUND_ENDPOINT_AUDIENCE;
pub(crate) const INTEGRATION_FAMILY: &str = "openai";
#[cfg(test)]
pub(crate) const SUPPORT_AUTHORITY: &str = "provider-supported-public-api";
