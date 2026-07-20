use crate::diagnostic::{ValueRequired, required_text};
use std::fmt;

macro_rules! text_identity {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
                required_text($field, value).map(Self)
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

text_identity!(IntegrationFamilyId, "integration family id");
text_identity!(TransportFamilyId, "transport family id");
text_identity!(ConfiguredInstanceId, "configured instance id");
text_identity!(InstanceRevision, "instance revision");
text_identity!(ExecutionHostId, "execution host id");
text_identity!(ModelRouteId, "model route id");
text_identity!(ModelRouteRevision, "model route revision");
text_identity!(AccessProfileId, "access profile id");
text_identity!(ProtocolFacadeId, "protocol facade id");
text_identity!(InstancePolicyId, "instance policy id");
text_identity!(EndpointAudience, "endpoint audience");

/// Host-owned reference to an endpoint, executable, SDK, or service.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InstanceTargetRef(String);

impl InstanceTargetRef {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("instance target reference", value).map(Self)
    }

    /// Passes the opaque reference back to the execution host.
    #[must_use]
    pub fn as_host_value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for InstanceTargetRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("InstanceTargetRef")
            .field(&"<opaque>")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExecutionLayer {
    HarnessInteraction,
    DirectModelInference,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperationShape {
    StructuredRun,
    InteractiveSession,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InstanceOwnership {
    ExternalAttached,
    HostOwnedEphemeral,
    HostOwnedPersistent,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DriverRole {
    Discovery,
    ModelCatalog,
    StructuredRun,
    InteractiveSession,
    ServingInstanceLifecycle,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HostServiceKind {
    Task,
    BlockingWork,
    Time,
    Process,
    Network,
    Credential,
    WorkingResource,
    Attachment,
    Schema,
    DiagnosticObserver,
}

#[cfg(test)]
mod tests {
    use super::{ConfiguredInstanceId, InstanceTargetRef};

    #[test]
    fn runtime_identity_rejects_blank_text() {
        let error = ConfiguredInstanceId::new("  ").expect_err("blank identity must fail");

        assert_eq!(error.field(), "configured instance id");
        assert_eq!(error.diagnostic().code(), "swallowtail.value_required");
    }

    #[test]
    fn host_reference_is_opaque_by_default() {
        let reference = InstanceTargetRef::new("/host/private/bin/provider")
            .expect("target reference is valid");

        assert_eq!(format!("{reference:?}"), "InstanceTargetRef(\"<opaque>\")");
        assert!(!format!("{reference:?}").contains(reference.as_host_value()));
    }
}
