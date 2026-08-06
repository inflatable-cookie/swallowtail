use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

/// Stable reason configured-instance catalogue admission failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConfiguredProviderInstanceCatalogueFailureKind {
    /// An instance, route, or model collection exceeds its portable bound.
    LimitExceeded,
    /// Two admitted records use the same configured-instance id.
    DuplicateInstance,
    /// One instance contains identical prepared route projections.
    DuplicateRoute,
    /// One model catalogue repeats the same provider and model identity.
    DuplicateModel,
    /// The configured instance does not belong to the supplied driver.
    DriverMismatch,
    /// The instance, profile, and prepared access evidence disagree.
    AccessMismatch,
    /// Prepared route evidence does not match its configured instance.
    RouteMismatch,
    /// The model-catalogue source is not among the admitted routes.
    ModelCatalogueSourceMissing,
    /// The model-catalogue source is not an unselected catalogue route.
    ModelCatalogueSourceInvalid,
}

/// Safe failure returned while admitting a configured-instance catalogue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguredProviderInstanceCatalogueFailure {
    kind: ConfiguredProviderInstanceCatalogueFailureKind,
    diagnostic: SafeDiagnostic,
}

impl ConfiguredProviderInstanceCatalogueFailure {
    fn new(
        kind: ConfiguredProviderInstanceCatalogueFailureKind,
        code: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
    /// Returns the stable failure classification.
    pub const fn kind(&self) -> ConfiguredProviderInstanceCatalogueFailureKind {
        self.kind
    }

    #[must_use]
    /// Returns the bounded, redacted diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ConfiguredProviderInstanceCatalogueFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ConfiguredProviderInstanceCatalogueFailure {}

pub(super) fn failure(
    kind: ConfiguredProviderInstanceCatalogueFailureKind,
    code: &'static str,
    message: &'static str,
) -> ConfiguredProviderInstanceCatalogueFailure {
    ConfiguredProviderInstanceCatalogueFailure::new(kind, code, message)
}
