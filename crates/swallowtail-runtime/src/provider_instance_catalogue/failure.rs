use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConfiguredProviderInstanceCatalogueFailureKind {
    LimitExceeded,
    DuplicateInstance,
    DuplicateRoute,
    DuplicateModel,
    DriverMismatch,
    AccessMismatch,
    RouteMismatch,
    ModelCatalogueSourceMissing,
    ModelCatalogueSourceInvalid,
}

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
    pub const fn kind(&self) -> ConfiguredProviderInstanceCatalogueFailureKind {
        self.kind
    }

    #[must_use]
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
