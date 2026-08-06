use crate::PreparedOperationEvidence;
use swallowtail_core::{ModelCatalogEntry, SafeDiagnostic};

use super::ConfiguredProviderInstanceRoute;

/// Availability state of a bound model-catalogue result.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConfiguredProviderModelCatalogueState {
    /// The source route returned a bounded model collection.
    Available,
    /// The source route failed with a safe diagnostic.
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ConfiguredProviderModelCatalogueOutcome {
    Available(Vec<ModelCatalogEntry>),
    Unavailable(SafeDiagnostic),
}

/// Authority-bearing model-catalogue result submitted for admission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderModelCatalogueInput {
    pub(super) source: PreparedOperationEvidence,
    pub(super) outcome: ConfiguredProviderModelCatalogueOutcome,
}

impl ConfiguredProviderModelCatalogueInput {
    /// Creates a successful result tied to its exact prepared source route.
    #[must_use]
    pub fn available(
        source: PreparedOperationEvidence,
        entries: impl IntoIterator<Item = ModelCatalogEntry>,
    ) -> Self {
        Self {
            source,
            outcome: ConfiguredProviderModelCatalogueOutcome::Available(
                entries.into_iter().collect(),
            ),
        }
    }

    #[must_use]
    /// Creates an unavailable result tied to its exact prepared source route.
    pub const fn unavailable(
        source: PreparedOperationEvidence,
        diagnostic: SafeDiagnostic,
    ) -> Self {
        Self {
            source,
            outcome: ConfiguredProviderModelCatalogueOutcome::Unavailable(diagnostic),
        }
    }
}

/// Safe model-catalogue projection bound to one admitted source route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderModelCatalogue {
    pub(super) source_route: ConfiguredProviderInstanceRoute,
    pub(super) outcome: ConfiguredProviderModelCatalogueOutcome,
}

impl ConfiguredProviderModelCatalogue {
    #[must_use]
    /// Returns the admitted route which produced this result.
    pub const fn source_route(&self) -> &ConfiguredProviderInstanceRoute {
        &self.source_route
    }

    #[must_use]
    /// Returns whether the source succeeded or was unavailable.
    pub const fn state(&self) -> ConfiguredProviderModelCatalogueState {
        match self.outcome {
            ConfiguredProviderModelCatalogueOutcome::Available(_) => {
                ConfiguredProviderModelCatalogueState::Available
            }
            ConfiguredProviderModelCatalogueOutcome::Unavailable(_) => {
                ConfiguredProviderModelCatalogueState::Unavailable
            }
        }
    }

    /// Iterates available models, or an empty collection when unavailable.
    pub fn entries(&self) -> impl ExactSizeIterator<Item = &ModelCatalogEntry> {
        match &self.outcome {
            ConfiguredProviderModelCatalogueOutcome::Available(entries) => entries.as_slice(),
            ConfiguredProviderModelCatalogueOutcome::Unavailable(_) => &[],
        }
        .iter()
    }

    #[must_use]
    /// Returns the safe failure diagnostic when the source was unavailable.
    pub const fn unavailable_diagnostic(&self) -> Option<&SafeDiagnostic> {
        match &self.outcome {
            ConfiguredProviderModelCatalogueOutcome::Available(_) => None,
            ConfiguredProviderModelCatalogueOutcome::Unavailable(diagnostic) => Some(diagnostic),
        }
    }

    pub(super) fn permits_selection(&self) -> bool {
        matches!(
            &self.outcome,
            ConfiguredProviderModelCatalogueOutcome::Available(entries) if !entries.is_empty()
        )
    }
}
