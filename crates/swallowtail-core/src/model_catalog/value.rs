use crate::diagnostic::SafeDiagnostic;
use crate::runtime_identity::IntegrationFamilyId;
use std::error::Error;
use std::fmt;

const MAX_PROVIDER_VALUE_BYTES: usize = 128;

/// One provider-defined catalogue value that is newer than common vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderCatalogValue {
    source: IntegrationFamilyId,
    value: String,
}

impl ProviderCatalogValue {
    pub fn new(
        source: IntegrationFamilyId,
        value: impl Into<String>,
    ) -> Result<Self, InvalidCatalogObservation> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(InvalidCatalogObservation::new(
                "swallowtail.catalog_value_required",
                "Provider catalogue value must not be empty",
            ));
        }
        if value.trim() != value || value.chars().any(char::is_control) {
            return Err(InvalidCatalogObservation::new(
                "swallowtail.catalog_value_unsafe",
                "Provider catalogue value contained an unsafe character",
            ));
        }
        if value.len() > MAX_PROVIDER_VALUE_BYTES {
            return Err(InvalidCatalogObservation::new(
                "swallowtail.catalog_value_too_long",
                "Provider catalogue value exceeded its bounded length",
            ));
        }
        Ok(Self { source, value })
    }

    #[must_use]
    pub const fn source(&self) -> &IntegrationFamilyId {
        &self.source
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// A common typed value or one bounded provider-defined extension.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CatalogObservation<T> {
    Known(T),
    ProviderDefined(ProviderCatalogValue),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidCatalogObservation {
    diagnostic: SafeDiagnostic,
}

impl InvalidCatalogObservation {
    pub(super) fn new(code: &'static str, message: &'static str) -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidCatalogObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidCatalogObservation {}
