//! Alibaba Model Studio Conversations and Responses direct-session driver.

#![forbid(unsafe_code)]

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
    AlibabaModelStudioPreparedEvidence, AlibabaModelStudioPreparedRun, AlibabaRunProfileInput,
};
pub use protocol::{
    ConversationInventory, ConversationRef, DeletionConfirmation, DeletionKind, ItemRef, Method,
    ProviderEvent, ResponseRef, ResponseStream, SseDecoder, SseFrame, TurnOptions, WireRequest,
    parse_conversation, parse_deletion, parse_inventory, parse_provider_failure,
    parse_request_correlation,
};
pub use selection::{
    ACCESS_PROFILE_ID, CONFIGURED_INSTANCE_ID, ENDPOINT_AUDIENCE, EVIDENCE_DATE, EXACT_MODEL_ID,
    MODEL_ROUTE_ID, REGION, WORKSPACE_ENDPOINT_TEMPLATE, alibaba_model_studio_access_profile,
    alibaba_model_studio_descriptor, alibaba_model_studio_instance,
    alibaba_model_studio_requirements, alibaba_model_studio_route,
    alibaba_model_studio_run_requirements, validate_alibaba_model_studio_plan,
};

pub const ALIBABA_DEPLOYABLE_MODELS_ENDPOINT: &str = "https://dashscope-intl.aliyuncs.com";
pub const ALIBABA_DEPLOYABLE_MODELS_ENDPOINT_AUDIENCE: &str = "dashscope-intl.aliyuncs.com";
pub const ALIBABA_DEPLOYABLE_MODELS_ACCESS_PROFILE_ID: &str =
    "alibaba-model-studio.intl.api-key.payg";
pub const ALIBABA_DEPLOYABLE_MODELS_CONFIGURED_INSTANCE_ID: &str =
    "alibaba-model-studio.intl.deployable-models";
pub const ALIBABA_DEPLOYABLE_MODELS_FACADE_REVISION: &str =
    "alibaba-deployable-models-v1.0-2026-06-06";

#[must_use]
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
pub fn alibaba_deployable_models_facade_claim() -> swallowtail_core::InterfaceCompatibilityClaim {
    swallowtail_core::InterfaceCompatibilityClaim::new(
        swallowtail_core::InterfaceCompatibilityClaimId::new("alibaba-deployable-models-window-1")
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
