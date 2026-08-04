use std::collections::BTreeSet;
use swallowtail_core::ConfiguredInstanceId;

use super::failure::failure;
use super::{
    ConfiguredProviderInstanceCatalogueFailure, ConfiguredProviderInstanceCatalogueFailureKind,
    ConfiguredProviderInstanceRecord, MAX_CONFIGURED_PROVIDER_INSTANCES,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderInstanceCatalogue {
    instances: Vec<ConfiguredProviderInstanceRecord>,
}

impl ConfiguredProviderInstanceCatalogue {
    pub fn new(
        instances: impl IntoIterator<Item = ConfiguredProviderInstanceRecord>,
    ) -> Result<Self, ConfiguredProviderInstanceCatalogueFailure> {
        let instances = instances.into_iter().collect::<Vec<_>>();
        if instances.len() > MAX_CONFIGURED_PROVIDER_INSTANCES {
            return Err(failure(
                ConfiguredProviderInstanceCatalogueFailureKind::LimitExceeded,
                "swallowtail.provider_instance_catalogue.instance_limit_exceeded",
                "Configured provider-instance catalogue exceeds its portable limit",
            ));
        }
        let mut ids = BTreeSet::new();
        if instances
            .iter()
            .any(|instance| !ids.insert(instance.instance_id().clone()))
        {
            return Err(failure(
                ConfiguredProviderInstanceCatalogueFailureKind::DuplicateInstance,
                "swallowtail.provider_instance_catalogue.instance_duplicate",
                "Configured provider-instance catalogue contains a duplicate instance",
            ));
        }
        Ok(Self { instances })
    }

    pub fn instances(&self) -> impl ExactSizeIterator<Item = &ConfiguredProviderInstanceRecord> {
        self.instances.iter()
    }

    #[must_use]
    pub fn get(
        &self,
        instance_id: &ConfiguredInstanceId,
    ) -> Option<&ConfiguredProviderInstanceRecord> {
        self.instances
            .iter()
            .find(|instance| instance.instance_id() == instance_id)
    }
}
