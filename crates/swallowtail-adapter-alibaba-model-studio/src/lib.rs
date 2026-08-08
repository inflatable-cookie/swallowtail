//! Alibaba Model Studio Conversations and Responses direct-session driver.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod activity;
mod catalogue;
mod driver;
mod failure;
mod prepared;
mod prepared_catalogue;
mod prepared_profile;
mod protocol;
mod selection;
mod transport;

pub use driver::AlibabaModelStudioDriver;
pub use failure::AlibabaProtocolFailure;
pub use prepared::{
    AlibabaModelStudioPreparationInput, AlibabaModelStudioPreparedIntegration,
    prepare_alibaba_model_studio,
};
pub use prepared_catalogue::{
    AlibabaDeployableModelsPreparationInput, AlibabaDeployableModelsPreparedIntegration,
    AlibabaDeployableModelsProfileInput, AlibabaPreparedDeployableModels,
    prepare_alibaba_deployable_models,
};
pub use prepared_profile::{
    AlibabaConversationProfileInput, AlibabaModelStudioPreparedConversation,
    AlibabaModelStudioPreparedDelete, AlibabaModelStudioPreparedEvidence,
    AlibabaModelStudioPreparedRetainedConversation, AlibabaModelStudioPreparedRun,
    AlibabaRetainedConversationProfileInput, AlibabaRunProfileInput, AlibabaSessionManagementInput,
};
pub use protocol::{
    ConversationInventory, ConversationMetadata, ConversationRef, ConversationReplayPage,
    DeletionConfirmation, DeletionKind, ItemRef, MAXIMUM_REPLAY_BYTES, MAXIMUM_REPLAY_ITEMS,
    MAXIMUM_REPLAY_PAGE_BYTES, MAXIMUM_REPLAY_PAGE_ITEMS, MAXIMUM_REPLAY_PAGES, Method,
    ProviderEvent, ResponseRef, ResponseStream, SseDecoder, SseFrame, TurnOptions, WireRequest,
    parse_conversation, parse_conversation_retrieval, parse_deletion, parse_inventory,
    parse_provider_failure, parse_replay_page, parse_request_correlation,
};
pub use selection::{
    ACCESS_PROFILE_ID, CONFIGURED_INSTANCE_ID, ENDPOINT_AUDIENCE, EVIDENCE_DATE, EXACT_MODEL_ID,
    MODEL_ROUTE_ID, REGION, WORKSPACE_ENDPOINT_TEMPLATE, alibaba_model_studio_access_profile,
    alibaba_model_studio_descriptor, alibaba_model_studio_facade_binding,
    alibaba_model_studio_facade_claim, alibaba_model_studio_instance,
    alibaba_model_studio_management_requirements, alibaba_model_studio_requirements,
    alibaba_model_studio_retained_requirements, alibaba_model_studio_route,
    alibaba_model_studio_run_requirements, validate_alibaba_model_studio_plan,
};

/// Exact international Model Studio origin used by deployable-model discovery.
pub const ALIBABA_DEPLOYABLE_MODELS_ENDPOINT: &str = "https://dashscope-intl.aliyuncs.com";
/// Credential audience for the international deployable-model catalogue.
pub const ALIBABA_DEPLOYABLE_MODELS_ENDPOINT_AUDIENCE: &str = "dashscope-intl.aliyuncs.com";
/// Canonical API-key access-profile identity for deployable-model discovery.
pub const ALIBABA_DEPLOYABLE_MODELS_ACCESS_PROFILE_ID: &str =
    "alibaba-model-studio.intl.api-key.payg";
/// Canonical configured-instance identity for deployable-model discovery.
pub const ALIBABA_DEPLOYABLE_MODELS_CONFIGURED_INSTANCE_ID: &str =
    "alibaba-model-studio.intl.deployable-models";
/// Exact qualified revision of the deployable-model catalogue facade.
pub const ALIBABA_DEPLOYABLE_MODELS_FACADE_REVISION: &str =
    "alibaba-deployable-models-v1.0-2026-06-06";

#[must_use]
/// Returns the exact interface binding for deployable-model discovery.
pub fn alibaba_deployable_models_facade_binding() -> swallowtail_core::InterfaceVersionBinding {
    swallowtail_core::InterfaceVersionBinding::new(
        swallowtail_core::InterfaceVersionAxis::new(
            "alibaba-model-studio.deployable-models-facade",
        )
        .expect("static axis is valid"),
        swallowtail_core::InterfaceVersion::new(ALIBABA_DEPLOYABLE_MODELS_FACADE_REVISION)
            .expect("static version is valid"),
    )
}

#[must_use]
/// Returns the qualified-only claim for deployable-model discovery.
pub fn alibaba_deployable_models_facade_claim() -> swallowtail_core::InterfaceCompatibilityClaim {
    swallowtail_core::InterfaceCompatibilityClaim::new(
        swallowtail_core::InterfaceCompatibilityClaimId::new("alibaba-model-studio.deployable-models-window-1")
            .expect("static claim id is valid"),
        swallowtail_core::InterfaceVersionAxis::new(
            "alibaba-model-studio.deployable-models-facade",
        )
        .expect("static axis is valid"),
        swallowtail_core::InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::exact(
            swallowtail_core::InterfaceVersion::new(ALIBABA_DEPLOYABLE_MODELS_FACADE_REVISION)
                .expect("static version is valid"),
            swallowtail_core::InterfaceBehaviorRevision::new("alibaba-deployable-models-list-v1")
                .expect("static behavior is valid"),
            swallowtail_core::InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static claim is valid")
}
pub use catalogue::{AlibabaDeployableModelsDriver, alibaba_deployable_models_descriptor};
