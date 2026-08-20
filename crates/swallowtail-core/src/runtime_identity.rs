use crate::diagnostic::{ValueRequired, required_text};
use std::fmt;

macro_rules! text_identity {
    ($name:ident, $field:literal) => {
        #[doc = concat!("Validated, non-empty ", $field, ".")]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates a ", $field, " after rejecting blank text.")]
            pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
                required_text($field, value).map(Self)
            }

            /// Returns the validated text.
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
    /// Creates an opaque target reference after rejecting blank text.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("instance target reference", value).map(Self)
    }

    /// Passes the opaque reference back to the execution host.
    #[must_use]
    pub fn as_host_value(&self) -> &str {
        &self.0
    }
}

/// Host-owned reference to one credential or delegated-auth context.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CredentialRef(String);

impl CredentialRef {
    /// Creates an opaque credential reference after rejecting blank text.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("credential reference", value).map(Self)
    }

    /// Passes the opaque reference back to the execution host.
    #[must_use]
    pub fn as_host_value(&self) -> &str {
        &self.0
    }
}

/// Host-owned reference to one per-instance config value.
///
/// The referenced path, URL, or environment body stays host-private.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConfigFieldRef(String);

impl ConfigFieldRef {
    /// Creates an opaque config-field reference after rejecting blank text.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("config field reference", value).map(Self)
    }

    /// Passes the opaque reference back to the execution host.
    #[must_use]
    pub fn as_host_value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for CredentialRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CredentialRef")
            .field(&"<opaque>")
            .finish()
    }
}

impl fmt::Debug for ConfigFieldRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ConfigFieldRef")
            .field(&"<opaque>")
            .finish()
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

/// Provider interaction layer used by one route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExecutionLayer {
    /// A provider or third-party agent harness owns model interaction.
    HarnessInteraction,
    /// Swallowtail calls a model inference API directly.
    DirectModelInference,
}

/// Runtime lifecycle shape exposed by one driver role.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperationShape {
    /// One bounded run produces one terminal outcome.
    StructuredRun,
    /// A reusable session accepts multiple turns.
    InteractiveSession,
    /// An inactive provider session is archived, restored, or deleted.
    ProviderSessionManagement,
    /// Provider sessions are listed without importing them.
    ProviderSessionCatalogue,
    /// One observed provider session is validated for ordinary attachment.
    ProviderSessionImport,
    /// An interrupted provider-session turn is observed after restart.
    ProviderSessionReconciliation,
    /// One newest-first page of provider-session history is read.
    ProviderSessionHistory,
    /// An interrupted retained run is observed after restart.
    ProviderRunReconciliation,
    /// A separately admitted recovered provider resource is cleaned up.
    ProviderRecoveredResourceCleanup,
}

/// Which boundary owns the configured provider instance.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InstanceOwnership {
    /// The runtime attaches to an independently owned instance.
    ExternalAttached,
    /// The host starts and stops an operation-scoped instance.
    HostOwnedEphemeral,
    /// The host owns an instance that persists across operations.
    HostOwnedPersistent,
}

/// Portable role implemented by a registered driver.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DriverRole {
    /// Observe an approved provider target and compatibility.
    Discovery,
    /// List models without selecting an inference operation.
    ModelCatalog,
    /// Execute one bounded run.
    StructuredRun,
    /// Open or attach to a reusable session.
    InteractiveSession,
    /// Open a realtime duplex media session.
    RealtimeMediaSession,
    /// Attach to or own a serving instance lifecycle.
    ServingInstanceLifecycle,
    /// Mutate an inactive provider session lifecycle.
    ProviderSessionManagement,
    /// List provider sessions.
    ProviderSessionCatalogue,
    /// Validate one provider session for attachment.
    ProviderSessionImport,
    /// Reconcile one interrupted provider-session turn.
    ProviderSessionReconciliation,
    /// Read one newest-first page of provider-session history.
    ProviderSessionHistory,
    /// Reconcile one interrupted retained run.
    ProviderRunReconciliation,
    /// Clean up an admitted recovered provider resource.
    ProviderRecoveredResourceCleanup,
}

/// Host service capability required by a preflight plan.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HostServiceKind {
    /// Scoped asynchronous task execution.
    Task,
    /// Scoped blocking-work execution.
    BlockingWork,
    /// Monotonic time and deadline observation.
    Time,
    /// Approved child-process execution.
    Process,
    /// Approved network access.
    Network,
    /// Credential lease resolution.
    Credential,
    /// Working-resource materialization.
    WorkingResource,
    /// Bounded working-resource reads and writes.
    WorkingResourceIo,
    /// Attachment materialization.
    Attachment,
    /// Model-artifact materialization.
    ModelArtifact,
    /// Serving-endpoint allocation or observation.
    ServingEndpoint,
    /// Structured-output schema materialization.
    Schema,
    /// Opt-in diagnostic and debug observation.
    DiagnosticObserver,
    /// Opt-in idiom selection source.
    IdiomSource,
    /// Opt-in fail-soft idiom signal recorder.
    IdiomRecorder,
    /// Open a host-approved URL for one interactive sign-in.
    UrlOpen,
    /// Bind a loopback callback for one sign-in operation.
    LoopbackCallback,
    /// Display a device code for one sign-in operation.
    DeviceCodeDisplay,
}

#[cfg(test)]
mod tests {
    use super::{ConfigFieldRef, ConfiguredInstanceId, InstanceTargetRef};

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

    #[test]
    fn config_field_reference_is_opaque_by_default() {
        let reference =
            ConfigFieldRef::new("/host/private/bin/provider").expect("config reference is valid");

        assert_eq!(format!("{reference:?}"), "ConfigFieldRef(\"<opaque>\")");
        assert!(!format!("{reference:?}").contains(reference.as_host_value()));
    }
}
