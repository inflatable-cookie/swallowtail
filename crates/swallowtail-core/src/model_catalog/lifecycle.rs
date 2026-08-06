use super::{CatalogObservation, InvalidCatalogObservation};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Common lifecycle status reported for a model.
pub enum ModelLifecycleStatus {
    /// Model is available for ordinary current use.
    Active,
    /// Model remains available but has entered a legacy phase.
    Legacy,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Common timestamped milestone in a model lifecycle.
pub enum ModelLifecycleTransition {
    /// Model first became available.
    StartOfLife,
    /// Model entered legacy status.
    Legacy,
    /// Model entered an extended-access phase.
    PublicExtendedAccess,
    /// Model reached its published end of life.
    EndOfLife,
}

/// Provider-reported time retained without a provider SDK date type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CatalogTimestamp {
    epoch_seconds: i64,
    subsecond_nanos: u32,
}

impl CatalogTimestamp {
    /// Creates a timestamp after validating the nanosecond fraction.
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
    /// Returns whole seconds from the Unix epoch.
    pub const fn epoch_seconds(&self) -> i64 {
        self.epoch_seconds
    }

    #[must_use]
    /// Returns the subsecond nanosecond fraction.
    pub const fn subsecond_nanos(&self) -> u32 {
        self.subsecond_nanos
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Provider-reported lifecycle status and its known transition times.
pub struct ModelLifecycleObservation {
    status: CatalogObservation<ModelLifecycleStatus>,
    transitions: BTreeMap<ModelLifecycleTransition, CatalogTimestamp>,
}

impl ModelLifecycleObservation {
    /// Starts lifecycle evidence with the current reported status.
    #[must_use]
    pub fn new(status: CatalogObservation<ModelLifecycleStatus>) -> Self {
        Self {
            status,
            transitions: BTreeMap::new(),
        }
    }

    #[must_use]
    /// Records or replaces one timestamped lifecycle transition.
    pub fn with_transition(
        mut self,
        transition: ModelLifecycleTransition,
        timestamp: CatalogTimestamp,
    ) -> Self {
        self.transitions.insert(transition, timestamp);
        self
    }

    #[must_use]
    /// Returns the current provider-reported lifecycle status.
    pub const fn status(&self) -> &CatalogObservation<ModelLifecycleStatus> {
        &self.status
    }

    #[must_use]
    /// Returns the time recorded for a lifecycle transition.
    pub fn transition(&self, transition: ModelLifecycleTransition) -> Option<CatalogTimestamp> {
        self.transitions.get(&transition).copied()
    }

    /// Iterates recorded lifecycle transitions in stable order.
    pub fn transitions(
        &self,
    ) -> impl ExactSizeIterator<Item = (&ModelLifecycleTransition, &CatalogTimestamp)> {
        self.transitions.iter()
    }
}
