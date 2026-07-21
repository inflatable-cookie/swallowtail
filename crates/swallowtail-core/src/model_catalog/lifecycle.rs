use super::{CatalogObservation, InvalidCatalogObservation};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModelLifecycleStatus {
    Active,
    Legacy,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModelLifecycleTransition {
    StartOfLife,
    Legacy,
    PublicExtendedAccess,
    EndOfLife,
}

/// Provider-reported time retained without a provider SDK date type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CatalogTimestamp {
    epoch_seconds: i64,
    subsecond_nanos: u32,
}

impl CatalogTimestamp {
    pub fn new(
        epoch_seconds: i64,
        subsecond_nanos: u32,
    ) -> Result<Self, InvalidCatalogObservation> {
        if subsecond_nanos >= 1_000_000_000 {
            return Err(InvalidCatalogObservation::new(
                "swallowtail.catalog_timestamp_invalid",
                "Catalogue timestamp nanoseconds were invalid",
            ));
        }
        Ok(Self {
            epoch_seconds,
            subsecond_nanos,
        })
    }

    #[must_use]
    pub const fn epoch_seconds(&self) -> i64 {
        self.epoch_seconds
    }

    #[must_use]
    pub const fn subsecond_nanos(&self) -> u32 {
        self.subsecond_nanos
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelLifecycleObservation {
    status: CatalogObservation<ModelLifecycleStatus>,
    transitions: BTreeMap<ModelLifecycleTransition, CatalogTimestamp>,
}

impl ModelLifecycleObservation {
    #[must_use]
    pub fn new(status: CatalogObservation<ModelLifecycleStatus>) -> Self {
        Self {
            status,
            transitions: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_transition(
        mut self,
        transition: ModelLifecycleTransition,
        timestamp: CatalogTimestamp,
    ) -> Self {
        self.transitions.insert(transition, timestamp);
        self
    }

    #[must_use]
    pub const fn status(&self) -> &CatalogObservation<ModelLifecycleStatus> {
        &self.status
    }

    #[must_use]
    pub fn transition(&self, transition: ModelLifecycleTransition) -> Option<CatalogTimestamp> {
        self.transitions.get(&transition).copied()
    }

    pub fn transitions(
        &self,
    ) -> impl ExactSizeIterator<Item = (&ModelLifecycleTransition, &CatalogTimestamp)> {
        self.transitions.iter()
    }
}
