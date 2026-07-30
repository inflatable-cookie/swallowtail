//! Kimi Platform direct-inference driver for Swallowtail.

#![forbid(unsafe_code)]

mod activity;
mod driver;
mod failure;
mod prepared;
mod prepared_profile;
mod protocol;
mod selection;
mod transport;

pub use driver::{KimiPlatformDirectDriver, kimi_platform_direct_descriptor};
pub use prepared::{
    KimiPlatformPreparationInput, KimiPlatformPreparedIntegration, prepare_kimi_platform_direct,
};
pub use prepared_profile::{
    KimiPlatformCatalogueProfileInput, KimiPlatformInferenceAttemptInput,
    KimiPlatformModelSelection, KimiPlatformPreparedCatalogue, KimiPlatformPreparedEvidence,
    KimiPlatformPreparedInferenceAttempt,
};
pub use selection::{
    KIMI_PLATFORM_ENDPOINT, KIMI_PLATFORM_ENDPOINT_AUDIENCE, KIMI_PLATFORM_FACADE_REVISION,
    KIMI_PLATFORM_MODEL_ID, KIMI_PLATFORM_PROVIDER_ID, kimi_platform_facade_binding,
    kimi_platform_facade_claim,
};
