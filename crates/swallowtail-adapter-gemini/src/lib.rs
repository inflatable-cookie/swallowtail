//! Gemini CLI, Developer API model-catalogue, and Live media integration.
//!
//! The installed CLI ACP and headless routes, hosted model catalogue, and
//! hosted realtime-media route are prepared independently. A prepared value
//! never implies that another Gemini route is configured or authorized.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod acp_activity;
mod catalogue;
mod connection;
mod discovery;
mod driver;
mod failure;
mod headless;
mod headless_activity;
mod headless_command;
mod headless_events;
mod headless_handle;
mod headless_pump;
mod headless_validation;
mod live;
mod live_compression;
mod live_protocol;
mod live_reasoning;
mod live_selection;
mod prepared;
mod prepared_catalogue;
mod prepared_cli;
mod prepared_headless;
mod prepared_live;
mod prepared_live_profile;
mod prepared_profile;
mod selection;
mod turn;

pub use driver::{GeminiAcpDriver, gemini_acp_descriptor};
pub use headless::{GeminiHeadlessDriver, gemini_headless_descriptor};
pub use live::{GeminiLiveDriver, gemini_live_descriptor};
pub use live_compression::GeminiLiveContextWindowCompression;
pub use live_selection::{
    GEMINI_LIVE_ACCESS_PROFILE_ID, GEMINI_LIVE_CONFIGURED_INSTANCE_ID, GEMINI_LIVE_ENDPOINT,
    GEMINI_LIVE_ENDPOINT_AUDIENCE, GEMINI_LIVE_FACADE_REVISION, GEMINI_LIVE_MAX_OUTPUT_TOKENS,
    GEMINI_LIVE_MODEL_ID, GEMINI_LIVE_MODEL_ROUTE_ID,
    GEMINI_LIVE_OUTPUT_MAXIMUM_SUPERSEDED_FACADE_REVISION, GEMINI_LIVE_SUPERSEDED_FACADE_REVISION,
    GEMINI_LIVE_THINKING_SUPERSEDED_FACADE_REVISION, gemini_live_access_profile,
    gemini_live_base_capabilities, gemini_live_facade_binding, gemini_live_facade_claim,
    gemini_live_instance, gemini_live_media_config, gemini_live_model_route,
    gemini_live_requirements, gemini_live_requirements_with_capabilities,
    gemini_live_rollover_policy,
};
pub use prepared::{
    GeminiPreparationInput, GeminiPreparationProbe, GeminiPreparedIntegration, prepare_gemini_acp,
};
pub use prepared_catalogue::{
    GeminiModelsPreparationInput, GeminiModelsPreparedIntegration, GeminiModelsProfileInput,
    GeminiPreparedModels, prepare_gemini_models,
};
pub use prepared_cli::{
    GeminiCliPreparationInput, GeminiCliPreparationProbe, GeminiCliPreparedDriver,
    GeminiCliPreparedIntegration, prepare_gemini_cli,
};
pub use prepared_headless::{
    GeminiHeadlessModelSelection, GeminiHeadlessPreparationInput, GeminiHeadlessPreparationProbe,
    GeminiHeadlessPreparedEvidence, GeminiHeadlessPreparedIntegration, GeminiHeadlessPreparedRun,
    GeminiHeadlessRunProfileInput, GeminiHeadlessRunRetention, prepare_gemini_headless,
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
    GEMINI_CLI_HEADLESS_AXIS, GEMINI_CLI_HEADLESS_BASELINE_VERSION,
    GEMINI_CLI_HEADLESS_LATEST_QUALIFIED_VERSION, gemini_cli_acp_binding, gemini_cli_acp_claim,
    gemini_cli_headless_binding, gemini_cli_headless_claim,
};

/// Exact Gemini Developer API origin used by the model-catalogue route.
pub const GEMINI_MODELS_ENDPOINT: &str = "https://generativelanguage.googleapis.com";
/// Credential audience required by the Gemini model-catalogue route.
pub const GEMINI_MODELS_ENDPOINT_AUDIENCE: &str = "generativelanguage.googleapis.com";
/// Access-profile identifier shared by the hosted Gemini API-key routes.
pub const GEMINI_MODELS_ACCESS_PROFILE_ID: &str = GEMINI_LIVE_ACCESS_PROFILE_ID;
/// Stable configured-instance identifier for the hosted model catalogue.
pub const GEMINI_MODELS_CONFIGURED_INSTANCE_ID: &str = "gemini.public.models";
/// Exact opaque facade revision for the Developer API models endpoint.
pub const GEMINI_MODELS_FACADE_REVISION: &str = "google.generativelanguage.v1beta.models.list";

/// Returns the exact interface binding for the hosted models facade.
#[must_use]
pub fn gemini_models_facade_binding() -> swallowtail_core::InterfaceVersionBinding {
    swallowtail_core::InterfaceVersionBinding::new(
        swallowtail_core::InterfaceVersionAxis::new("gemini.models-facade")
            .expect("static axis is valid"),
        swallowtail_core::InterfaceVersion::new(GEMINI_MODELS_FACADE_REVISION)
            .expect("static version is valid"),
    )
}

/// Returns the qualified compatibility claim for the hosted models facade.
#[must_use]
pub fn gemini_models_facade_claim() -> swallowtail_core::InterfaceCompatibilityClaim {
    swallowtail_core::InterfaceCompatibilityClaim::new(
        swallowtail_core::InterfaceCompatibilityClaimId::new("gemini.models-window-1")
            .expect("static claim id is valid"),
        swallowtail_core::InterfaceVersionAxis::new("gemini.models-facade")
            .expect("static axis is valid"),
        swallowtail_core::InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::exact(
            swallowtail_core::InterfaceVersion::new(GEMINI_MODELS_FACADE_REVISION)
                .expect("static version is valid"),
            swallowtail_core::InterfaceBehaviorRevision::new("gemini-models-list-v1")
                .expect("static behavior is valid"),
            swallowtail_core::InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static claim is valid")
}
pub use catalogue::{GeminiModelsDriver, gemini_models_descriptor};
