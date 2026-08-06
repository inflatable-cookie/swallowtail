#![deny(missing_docs)]

mod admission;
mod catalogue;
mod credential;
mod failure;
mod instance;
mod model;
mod route;
mod validation;

pub use admission::ConfiguredProviderInstanceAdmission;
pub use catalogue::ConfiguredProviderInstanceCatalogue;
pub use credential::ConfiguredProviderCredentialPosture;
pub use failure::{
    ConfiguredProviderInstanceCatalogueFailure, ConfiguredProviderInstanceCatalogueFailureKind,
};
pub use instance::{
    ConfiguredProviderInstanceRecord, ConfiguredProviderInstanceSelectionReadiness,
};
pub use model::{
    ConfiguredProviderModelCatalogue, ConfiguredProviderModelCatalogueInput,
    ConfiguredProviderModelCatalogueState,
};
pub use route::{ConfiguredProviderInstanceRoute, ConfiguredProviderModelRoute};

/// Maximum configured instances in one portable catalogue snapshot.
pub const MAX_CONFIGURED_PROVIDER_INSTANCES: usize = 256;
/// Maximum prepared operation routes projected for one configured instance.
pub const MAX_CONFIGURED_PROVIDER_ROUTES_PER_INSTANCE: usize = 64;
/// Maximum model entries projected for one configured instance.
pub const MAX_CONFIGURED_PROVIDER_MODELS_PER_INSTANCE: usize = 10_000;

#[cfg(test)]
#[path = "provider_instance_catalogue/tests.rs"]
mod tests;
