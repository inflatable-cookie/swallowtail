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

/// Stable reason readiness refresh failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReadinessRefreshFailureKind {
    /// The admitted instance is absent from the store.
    InstanceAbsent,
    /// The store rejected the refreshed record.
    Store,
}

/// Rejection raised while refreshing access dimensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadinessRefreshFailure {
    kind: ReadinessRefreshFailureKind,
    diagnostic: SafeDiagnostic,
}

impl ReadinessRefreshFailure {
    pub(super) fn instance_absent() -> Self {
        Self {
            kind: ReadinessRefreshFailureKind::InstanceAbsent,
            diagnostic: SafeDiagnostic::new(
                "swallowtail.connection_lifecycle.refresh_instance_absent",
                "Readiness refresh requires an admitted instance in the store",
            ),
        }
    }

    pub(super) fn from_store(failure: ConnectionLifecycleStoreFailure) -> Self {
        Self {
            kind: ReadinessRefreshFailureKind::Store,
            diagnostic: failure.diagnostic().clone(),
        }
    }

    #[must_use]
    /// Returns the stable failure classification.
    pub const fn kind(&self) -> ReadinessRefreshFailureKind {
        self.kind
    }

    #[must_use]
    /// Returns the redacted refresh diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ReadinessRefreshFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ReadinessRefreshFailure {}

/// Stable reason authenticated-subject observation failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SubjectObservationFailureKind {
    /// The admitted instance is absent from the store.
    InstanceAbsent,
    /// The store rejected the instance lookup.
    Store,
}

/// Rejection raised while observing an authenticated subject.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubjectObservationFailure {
    kind: SubjectObservationFailureKind,
    diagnostic: SafeDiagnostic,
}

impl SubjectObservationFailure {
    pub(super) fn instance_absent() -> Self {
        Self {
            kind: SubjectObservationFailureKind::InstanceAbsent,
            diagnostic: SafeDiagnostic::new(
                "swallowtail.connection_lifecycle.subject_instance_absent",
                "Authenticated-subject observation requires an admitted instance in the store",
            ),
        }
    }

    pub(super) fn from_store(failure: ConnectionLifecycleStoreFailure) -> Self {
        Self {
            kind: SubjectObservationFailureKind::Store,
            diagnostic: failure.diagnostic().clone(),
        }
    }

    #[must_use]
    /// Returns the stable failure classification.
    pub const fn kind(&self) -> SubjectObservationFailureKind {
        self.kind
    }

    #[must_use]
    /// Returns the redacted subject-observation diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for SubjectObservationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for SubjectObservationFailure {}
