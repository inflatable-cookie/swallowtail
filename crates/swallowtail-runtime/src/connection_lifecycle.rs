//! Connection-lifecycle catalog, admission, store port, and sign-in loop for
//! Contract 057.
//!
//! Runtime owns catalog assembly, instance admission, the persistence trait,
//! and library-owned sign-in. Optional in-memory, JSON-file, and sign-in
//! test-double adapters live in `swallowtail-host-local`. This surface never
//! requires raw secrets and does not project 047 readiness.

mod admission;
mod catalog;
mod failure;
mod sign_in;

use std::error::Error;
use std::fmt;
use swallowtail_core::{
    AdmittedInstanceRecord, ConfiguredInstanceId, OverlayMarker, SafeDiagnostic,
};

pub use admission::{InstanceAdmissionRequest, admit_instance};
pub use catalog::AddableRouteCatalog;
pub use failure::{
    AddableRouteCatalogFailure, AddableRouteCatalogFailureKind, InstanceAdmissionFailure,
    InstanceAdmissionFailureKind,
};
pub use sign_in::{
    SignInAuthorityBinding, SignInFailure, SignInFailureKind, SignInKind, SignInMethod,
    SignInOutcome, SignInSession, SignInStartRequest, SignInStatus, cancel_sign_in,
    complete_sign_in, poll_sign_in, start_sign_in, submit_sign_in_credential_field,
};

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
#[path = "connection_lifecycle/sign_in_tests.rs"]
mod sign_in_tests;
#[cfg(test)]
#[path = "connection_lifecycle/tests.rs"]
mod tests;
