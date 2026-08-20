use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

use super::ConnectionLifecycleStoreFailure;

/// Stable reason addable-route catalog assembly failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AddableRouteCatalogFailureKind {
    /// Two descriptors used the same addable-route id.
    DuplicateRoute,
}

/// Rejection raised while assembling an addable-route catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddableRouteCatalogFailure {
    kind: AddableRouteCatalogFailureKind,
    diagnostic: SafeDiagnostic,
}

impl AddableRouteCatalogFailure {
    pub(super) fn duplicate_route() -> Self {
        Self {
            kind: AddableRouteCatalogFailureKind::DuplicateRoute,
            diagnostic: SafeDiagnostic::new(
                "swallowtail.connection_lifecycle.duplicate_route",
                "Addable-route catalog contains a duplicate route",
            ),
        }
    }

    #[must_use]
    /// Returns the stable failure classification.
    pub const fn kind(&self) -> AddableRouteCatalogFailureKind {
        self.kind
    }

    #[must_use]
    /// Returns the redacted catalog diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for AddableRouteCatalogFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for AddableRouteCatalogFailure {}

/// Stable reason instance admission failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InstanceAdmissionFailureKind {
    /// The catalog does not contain this addable route.
    RouteAbsent,
    /// The route is known but a named requirement is missing.
    RouteUnavailable,
    /// The adapter will not offer this route on this host.
    RouteUnsupported,
    /// A credential reference does not match an advertised field.
    UnknownCredentialField,
    /// A config-field reference does not match an advertised field.
    UnknownConfigField,
    /// The store rejected the admitted record.
    Store,
}

/// Rejection raised while admitting a configured instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstanceAdmissionFailure {
    kind: InstanceAdmissionFailureKind,
    diagnostic: SafeDiagnostic,
}

impl InstanceAdmissionFailure {
    fn new(kind: InstanceAdmissionFailureKind, code: &'static str, message: &'static str) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    pub(super) fn route_absent() -> Self {
        Self::new(
            InstanceAdmissionFailureKind::RouteAbsent,
            "swallowtail.connection_lifecycle.route_absent",
            "Addable route is absent from the assembled catalog",
        )
    }

    pub(super) fn route_unavailable() -> Self {
        Self::new(
            InstanceAdmissionFailureKind::RouteUnavailable,
            "swallowtail.connection_lifecycle.route_unavailable",
            "Addable route is unavailable on this host",
        )
    }

    pub(super) fn route_unsupported() -> Self {
        Self::new(
            InstanceAdmissionFailureKind::RouteUnsupported,
            "swallowtail.connection_lifecycle.route_unsupported",
            "Addable route is unsupported on this host",
        )
    }

    pub(super) fn unknown_credential_field() -> Self {
        Self::new(
            InstanceAdmissionFailureKind::UnknownCredentialField,
            "swallowtail.connection_lifecycle.unknown_credential_field",
            "Admission credential reference does not match an advertised field",
        )
    }

    pub(super) fn unknown_config_field() -> Self {
        Self::new(
            InstanceAdmissionFailureKind::UnknownConfigField,
            "swallowtail.connection_lifecycle.unknown_config_field",
            "Admission config-field reference does not match an advertised field",
        )
    }

    pub(super) fn from_store(failure: ConnectionLifecycleStoreFailure) -> Self {
        Self {
            kind: InstanceAdmissionFailureKind::Store,
            diagnostic: failure.diagnostic().clone(),
        }
    }

    #[must_use]
    /// Returns the stable failure classification.
    pub const fn kind(&self) -> InstanceAdmissionFailureKind {
        self.kind
    }

    #[must_use]
    /// Returns the redacted admission diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InstanceAdmissionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InstanceAdmissionFailure {}
