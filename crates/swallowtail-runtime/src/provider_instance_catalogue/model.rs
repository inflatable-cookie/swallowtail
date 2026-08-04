use crate::PreparedOperationEvidence;
use swallowtail_core::{ModelCatalogEntry, SafeDiagnostic};

use super::ConfiguredProviderInstanceRoute;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConfiguredProviderModelCatalogueState {
    Available,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ConfiguredProviderModelCatalogueOutcome {
    Available(Vec<ModelCatalogEntry>),
    Unavailable(SafeDiagnostic),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderModelCatalogueInput {
    pub(super) source: PreparedOperationEvidence,
    pub(super) outcome: ConfiguredProviderModelCatalogueOutcome,
}

impl ConfiguredProviderModelCatalogueInput {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderModelCatalogue {
    pub(super) source_route: ConfiguredProviderInstanceRoute,
    pub(super) outcome: ConfiguredProviderModelCatalogueOutcome,
}

impl ConfiguredProviderModelCatalogue {
    #[must_use]
    pub const fn source_route(&self) -> &ConfiguredProviderInstanceRoute {
        &self.source_route
    }

    #[must_use]
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

    pub fn entries(&self) -> impl ExactSizeIterator<Item = &ModelCatalogEntry> {
        match &self.outcome {
            ConfiguredProviderModelCatalogueOutcome::Available(entries) => entries.as_slice(),
            ConfiguredProviderModelCatalogueOutcome::Unavailable(_) => &[],
        }
        .iter()
    }

    #[must_use]
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
