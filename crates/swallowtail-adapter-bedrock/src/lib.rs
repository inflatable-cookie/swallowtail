//! Amazon Bedrock Runtime integration through the official AWS SDK for Rust.

#![forbid(unsafe_code)]

mod binding;
mod catalogue;
mod driver;
mod failure;
mod sdk;
mod sdk_config;
mod stream;

pub use binding::{BedrockCredentialProvider, BedrockDriverBinding, BedrockRegion};
pub use catalogue::{
    BedrockCatalogueBinding, BedrockCatalogueDriver, bedrock_catalogue_descriptor,
};
pub use driver::{BedrockDirectDriver, bedrock_direct_descriptor};
pub use stream::{
    DecodeFailure, ProviderFailureKind, StopKind, StreamDecoder, StreamUpdate, TokenUsage,
    classify_converse_failure, classify_output_failure,
};

pub const SDK_CRATE: &str = "aws-sdk-bedrockruntime";
pub const SDK_VERSION: &str = "1.136.0";
pub const SERVICE_API: &str = "Amazon Bedrock Runtime ConverseStream";
pub const CATALOGUE_SDK_CRATE: &str = "aws-sdk-bedrock";
pub const CATALOGUE_SDK_VERSION: &str = "1.148.0";
pub const CATALOGUE_SERVICE_API: &str = "Amazon Bedrock ListFoundationModels";
