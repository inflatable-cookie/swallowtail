//! Amazon Bedrock Runtime integration through the official AWS SDK for Rust.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod activity;
mod binding;
mod catalogue;
mod consumer_route_projection;
mod driver;
mod failure;
mod prepared;
mod sdk;
mod sdk_config;
mod selection;
mod stream;

pub use binding::{
    BedrockCloudClientConfig, BedrockCredentialProvider, BedrockDriverBinding, BedrockRegion,
};
pub use catalogue::{
    BedrockCatalogueBinding, BedrockCatalogueDriver, bedrock_catalogue_descriptor,
};
pub use driver::{BedrockDirectDriver, bedrock_direct_descriptor};
pub use prepared::{
    BedrockCataloguePreparationInput, BedrockCataloguePreparedEvidence,
    BedrockCataloguePreparedIntegration, BedrockCatalogueProfileInput, BedrockCatalogueRouteInput,
    BedrockFacade, BedrockFacadePreparationInput, BedrockModelSelection, BedrockPreparedCatalogue,
    BedrockPreparedInferenceAttempt, BedrockRuntimePreparationInput,
    BedrockRuntimePreparedEvidence, BedrockRuntimePreparedIntegration, BedrockRuntimeProfileInput,
    BedrockRuntimeRouteInput, prepare_bedrock, prepare_bedrock_catalogue, prepare_bedrock_runtime,
};
pub use selection::{
    BEDROCK_CATALOGUE_ACCESS_PROFILE_ID, BEDROCK_CATALOGUE_FACADE_REVISION,
    BEDROCK_CATALOGUE_INSTANCE_POLICY_ID, BEDROCK_CATALOGUE_SERVICE_REVISION,
    BEDROCK_CONTROL_PLANE_ENDPOINT_AUDIENCE, BEDROCK_RUNTIME_ACCESS_PROFILE_ID,
    BEDROCK_RUNTIME_ENDPOINT_AUDIENCE, BEDROCK_RUNTIME_FACADE_REVISION,
    BEDROCK_RUNTIME_INSTANCE_POLICY_ID, BEDROCK_RUNTIME_SERVICE_REVISION,
    bedrock_catalogue_access_profile, bedrock_catalogue_interface_bindings,
    bedrock_catalogue_interface_claims, bedrock_runtime_access_profile,
    bedrock_runtime_interface_bindings, bedrock_runtime_interface_claims,
};
pub use stream::{
    DecodeFailure, ProviderFailureKind, StopKind, StreamDecoder, StreamUpdate, TokenUsage,
    classify_converse_failure, classify_output_failure,
};

/// AWS SDK crate used for Bedrock Runtime inference.
pub const SDK_CRATE: &str = "aws-sdk-bedrockruntime";
/// Exact AWS SDK version qualified for Bedrock Runtime.
pub const SDK_VERSION: &str = "1.136.0";
/// Bedrock Runtime service operation implemented by this adapter.
pub const SERVICE_API: &str = "Amazon Bedrock Runtime ConverseStream";
/// AWS SDK crate used for Bedrock control-plane catalogue discovery.
pub const CATALOGUE_SDK_CRATE: &str = "aws-sdk-bedrock";
/// Exact AWS SDK version qualified for the control-plane catalogue.
pub const CATALOGUE_SDK_VERSION: &str = "1.148.0";
/// Bedrock control-plane service operation implemented by this adapter.
pub const CATALOGUE_SERVICE_API: &str = "Amazon Bedrock ListFoundationModels";
