//! xAI direct-inference drivers for Swallowtail.
//!
//! The adapter owns a serial, resource-free Responses WebSocket session. It
//! does not enable provider storage, reconnect, retry, or durable resume.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod activity;
mod catalogue;
mod consumer_route_projection;
mod controls;
mod driver;
mod failure;
mod prepared;
mod prepared_catalogue;
mod prepared_profile;
mod protocol;
mod selection;
mod transport;

pub use driver::{XaiWebSocketDriver, xai_websocket_descriptor};
pub use prepared::{XaiPreparationInput, XaiPreparedIntegration, prepare_xai_responses_websocket};
pub use prepared_catalogue::{
    XaiModelsPreparationInput, XaiModelsPreparedIntegration, XaiModelsProfileInput,
    XaiPreparedModels, prepare_xai_models,
};
pub use prepared_profile::{
    XaiModelSelection, XaiPreparedEvidence, XaiPreparedResponsesRun, XaiPreparedResponsesSession,
    XaiRunProfileInput, XaiSessionProfileInput,
};
pub use selection::{
    XAI_RESPONSES_ACCESS_PROFILE_ID, XAI_RESPONSES_CONFIGURED_INSTANCE_ID, XAI_RESPONSES_ENDPOINT,
    XAI_RESPONSES_ENDPOINT_AUDIENCE, XAI_RESPONSES_FACADE_REVISION, xai_responses_access_profile,
    xai_responses_facade_binding, xai_responses_facade_claim, xai_responses_instance,
    xai_responses_model_route, xai_responses_requirements, xai_responses_run_requirements,
};

/// Provider-supported Responses WebSocket route frozen by the first fixture.
pub const RESPONSES_WEBSOCKET_PATH: &str = "/v1/responses";

/// Exact scale used by xAI's provider-authored billed-cost observation.
pub const USD_TICKS_PER_USD: u64 = 10_000_000_000;

/// Official xAI origin used by the read-only models route.
pub const XAI_MODELS_ENDPOINT: &str = "https://api.x.ai";
/// Endpoint audience required by xAI Models credential leases.
pub const XAI_MODELS_ENDPOINT_AUDIENCE: &str = XAI_RESPONSES_ENDPOINT_AUDIENCE;
/// Public API-key profile admitted by the xAI Models route.
pub const XAI_MODELS_ACCESS_PROFILE_ID: &str = XAI_RESPONSES_ACCESS_PROFILE_ID;
/// Stable configured-instance identity for the xAI Models route.
pub const XAI_MODELS_CONFIGURED_INSTANCE_ID: &str = "xai.public.language-models";
/// Exact opaque xAI Models facade revision.
pub const XAI_MODELS_FACADE_REVISION: &str = "xai-language-models-2026-07-27";

#[must_use]
/// Returns the exact xAI Models facade version binding.
pub fn xai_models_facade_binding() -> swallowtail_core::InterfaceVersionBinding {
    swallowtail_core::InterfaceVersionBinding::new(
        swallowtail_core::InterfaceVersionAxis::new("xai.models-facade")
            .expect("static axis is valid"),
        swallowtail_core::InterfaceVersion::new(XAI_MODELS_FACADE_REVISION)
            .expect("static version is valid"),
    )
}

#[must_use]
/// Returns the qualified-only compatibility claim for the Models facade.
pub fn xai_models_facade_claim() -> swallowtail_core::InterfaceCompatibilityClaim {
    swallowtail_core::InterfaceCompatibilityClaim::new(
        swallowtail_core::InterfaceCompatibilityClaimId::new("xai.models-window-1")
            .expect("static claim id is valid"),
        swallowtail_core::InterfaceVersionAxis::new("xai.models-facade")
            .expect("static axis is valid"),
        swallowtail_core::InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::exact(
            swallowtail_core::InterfaceVersion::new(XAI_MODELS_FACADE_REVISION)
                .expect("static version is valid"),
            swallowtail_core::InterfaceBehaviorRevision::new("xai-language-models-list-v1")
                .expect("static behavior is valid"),
            swallowtail_core::InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static claim is valid")
}
pub use catalogue::{XaiModelsDriver, xai_models_descriptor};
