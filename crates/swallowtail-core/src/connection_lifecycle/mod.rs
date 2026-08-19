//! Portable Contract 057 records for the pre-session connection lifecycle.
//!
//! These types carry descriptors, admitted-instance store records, overlay
//! markers, and redacted subject observations. They do not persist secrets,
//! assemble a catalog, or project 047 readiness.

mod descriptor;
mod identity;
mod records;

pub use descriptor::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteMissingRequirement,
    ConfigFieldDescriptor, ConfigFieldKind, CredentialFieldDescriptor, CredentialFieldVisibility,
    RouteTopology,
};
pub use identity::{
    AddableRouteId, ConfigFieldId, CredentialFieldId, EnvironmentVariableName, FieldLabel,
    InstanceLabel,
};
pub use records::{
    AdmittedInstanceRecord, AuthenticatedSubjectObservation, InstanceEnablement, OverlayMarker,
    SubjectDisclosure,
};

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
