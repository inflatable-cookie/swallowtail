use crate::{PreparedAccessEvidence, PreparedOperationEvidence};
use swallowtail_core::{AccessProfile, ConfiguredInstance, DriverDescriptor};

use super::ConfiguredProviderModelCatalogueInput;

/// Authority-bearing input used to project one safe configured-instance record.
///
/// Admission validates every supplied route and optional model catalogue before
/// discarding executable handles, credentials, and raw target references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderInstanceAdmission {
    pub(super) driver: DriverDescriptor,
    pub(super) instance: ConfiguredInstance,
    pub(super) access_profile: AccessProfile,
    pub(super) access_evidence: PreparedAccessEvidence,
    pub(super) prepared_routes: Vec<PreparedOperationEvidence>,
    pub(super) model_catalogue: Option<ConfiguredProviderModelCatalogueInput>,
}

impl ConfiguredProviderInstanceAdmission {
    /// Creates admission input for an exact driver, instance, and access profile.
    #[must_use]
    pub const fn new(
        driver: DriverDescriptor,
        instance: ConfiguredInstance,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            driver,
            instance,
            access_profile,
            access_evidence,
            prepared_routes: Vec::new(),
            model_catalogue: None,
        }
    }

    #[must_use]
    /// Adds the exact prepared routes to expose as non-executable evidence.
    pub fn with_prepared_routes(
        mut self,
        routes: impl IntoIterator<Item = PreparedOperationEvidence>,
    ) -> Self {
        self.prepared_routes = routes.into_iter().collect();
        self
    }

    #[must_use]
    /// Adds an available or unavailable model-catalogue result and its source.
    pub fn with_model_catalogue(
        mut self,
        catalogue: ConfiguredProviderModelCatalogueInput,
    ) -> Self {
        self.model_catalogue = Some(catalogue);
        self
    }
}
