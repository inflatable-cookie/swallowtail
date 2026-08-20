//! Connection-lifecycle catalog, admission, store port, sign-in loop,
//! readiness refresh, subject observation, and update observation for
//! Contract 057.
//!
//! Runtime owns catalog assembly, instance admission, the persistence trait,
//! library-owned sign-in, refresh of access dimensions, optional subject
//! observation, and derived update observation. Optional in-memory, JSON-file,
//! and sign-in test-double adapters live in `swallowtail-host-local`. This
//! surface never requires raw secrets and does not project 047 readiness.

mod admission;
mod catalog;
mod failure;
mod refresh;
mod sign_in;
mod subject;
mod update;

use std::error::Error;
use std::fmt;
use swallowtail_core::{
    AdmittedInstanceRecord, ConfiguredInstanceId, OverlayMarker, SafeDiagnostic,
};

pub use admission::{InstanceAdmissionRequest, admit_instance};
pub use catalog::AddableRouteCatalog;
pub use failure::{
    AddableRouteCatalogFailure, AddableRouteCatalogFailureKind, InstanceAdmissionFailure,
    InstanceAdmissionFailureKind, ReadinessRefreshFailure, ReadinessRefreshFailureKind,
    SubjectObservationFailure, SubjectObservationFailureKind,
};
pub use refresh::{ReadinessRefreshRequest, refresh_readiness};
pub use sign_in::{
    SignInAuthorityBinding, SignInFailure, SignInFailureKind, SignInKind, SignInMethod,
    SignInOutcome, SignInSession, SignInStartRequest, SignInStatus, cancel_sign_in,
    complete_sign_in, poll_sign_in, start_sign_in, submit_sign_in_credential_field,
};
pub use subject::observe_authenticated_subject;
pub use update::observe_instance_update;

/// Rejection raised by a connection-lifecycle store adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConnectionLifecycleStoreFailure {
    diagnostic: SafeDiagnostic,
}

impl ConnectionLifecycleStoreFailure {
    /// Creates a store failure from a redacted diagnostic.
    #[must_use]
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
    /// Returns the redacted store diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ConnectionLifecycleStoreFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ConnectionLifecycleStoreFailure {}

/// Persistence port for admitted instances, secret references, enablement,
/// labels, and overlay markers.
///
/// Implementations must not require raw secret bytes. Enablement is a host
/// preference independent of stored access-status dimensions.
pub trait ConnectionLifecycleStore: Send + Sync {
    /// Inserts or replaces one admitted instance record.
    fn put_instance(
        &self,
        record: AdmittedInstanceRecord,
    ) -> Result<(), ConnectionLifecycleStoreFailure>;

    /// Returns the instance with this configured-instance id, when present.
    fn get_instance(
        &self,
        id: &ConfiguredInstanceId,
    ) -> Result<Option<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure>;

    /// Lists admitted instances in stable configured-instance id order.
    fn list_instances(
        &self,
    ) -> Result<Vec<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure>;

    /// Inserts or replaces one overlay marker keyed by instance, provider, and model.
    fn put_overlay_marker(
        &self,
        marker: OverlayMarker,
    ) -> Result<(), ConnectionLifecycleStoreFailure>;

    /// Lists overlay markers in stable instance, provider, and model order.
    fn list_overlay_markers(&self) -> Result<Vec<OverlayMarker>, ConnectionLifecycleStoreFailure>;
}

#[cfg(test)]
#[path = "connection_lifecycle/admission_tests.rs"]
mod admission_tests;
#[cfg(test)]
#[path = "connection_lifecycle/catalog_tests.rs"]
mod catalog_tests;
#[cfg(test)]
#[path = "connection_lifecycle/refresh_tests.rs"]
mod refresh_tests;
#[cfg(test)]
#[path = "connection_lifecycle/sign_in_tests.rs"]
mod sign_in_tests;
#[cfg(test)]
#[path = "connection_lifecycle/subject_tests.rs"]
mod subject_tests;
#[cfg(test)]
#[path = "connection_lifecycle/tests.rs"]
mod tests;
#[cfg(test)]
#[path = "connection_lifecycle/update_tests.rs"]
mod update_tests;
