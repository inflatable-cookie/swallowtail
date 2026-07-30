//! DeepSeek direct-inference integration for Swallowtail.
//!
//! The adapter owns one exact resource-free V4 Pro continuation session. Tool
//! execution and the decision to continue remain with the consumer.

#![forbid(unsafe_code)]

mod activity;
mod driver;
mod failure;
mod prepared;
mod prepared_profile;
mod protocol;
mod selection;
mod transport;

pub use driver::{DeepSeekDirectDriver, deepseek_direct_descriptor};
pub use prepared::{
    DeepSeekPreparationInput, DeepSeekPreparedIntegration, prepare_deepseek_direct,
};
pub use prepared_profile::{
    DeepSeekCatalogueProfileInput, DeepSeekModelSelection, DeepSeekPreparedCatalogue,
    DeepSeekPreparedEvidence, DeepSeekPreparedRun, DeepSeekPreparedSession,
    DeepSeekRunProfileInput, DeepSeekSessionProfileInput,
};
pub use selection::{
    DEEPSEEK_ENDPOINT, DEEPSEEK_ENDPOINT_AUDIENCE, DEEPSEEK_FACADE_REVISION, DEEPSEEK_MODEL_ID,
    deepseek_facade_binding, deepseek_facade_claim, deepseek_v4_config, deepseek_v4_requirements,
    deepseek_v4_run_requirements, validate_deepseek_request_plan,
};
