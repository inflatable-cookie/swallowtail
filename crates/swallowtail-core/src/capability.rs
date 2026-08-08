#![deny(missing_docs)]

use crate::diagnostic::SafeDiagnostic;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

/// A provider feature which a host must check before use.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Capability {
    /// Enumerate models available through a route.
    ModelCatalog,
    /// Maintain a multi-turn interactive session.
    InteractiveSession,
    /// Execute a bounded structured run.
    StructuredRun,
    /// Emit incremental operation events.
    StreamingEvents,
    /// Project provider-visible work as portable activity.
    ObservableActivity,
    /// Request or execute tool calls through the declared route boundary.
    ToolCalls,
    /// Interrupt an active operation.
    Interruption,
    /// Detach local observation while provider work remains active.
    ActiveOperationDetachment,
    /// Load retained provider-session state.
    LoadSession,
    /// Continue a loaded or retained provider session.
    Resume,
    /// Recover usability by attaching to an exact provider session.
    ProviderSessionAttachmentRecovery,
    /// Enforce a requested structured-output shape.
    StructuredOutput,
    /// Accept non-text request attachments.
    Attachments,
    /// Select an explicit reasoning mode or effort.
    ReasoningSelection,
    /// Select a provider harness operating mode.
    HarnessModeSelection,
    /// Bind the operation to a working resource.
    WorkingResource,
    /// Write bounded text through the working-resource service.
    WorkingResourceTextWrite,
    /// Allow provider-side external network access.
    ProviderExternalNetwork,
    /// Request external search through the route.
    ExternalSearch,
    /// Report provider usage measurements.
    UsageReporting,
    /// Report billed monetary cost.
    BilledCostReporting,
    /// Apply an output-token ceiling.
    OutputTokenLimit,
    /// Continue execution in provider-owned background state.
    ProviderBackgroundExecution,
    /// Permit temporary provider retention.
    ProviderTemporaryRetention,
    /// Permit durable provider retention.
    ProviderDurableRetention,
    /// Recover provider-managed execution state.
    ProviderManagedRecovery,
    /// Archive a retained provider session.
    ProviderSessionArchive,
    /// Restore an archived provider session.
    ProviderSessionRestore,
    /// Delete retained provider-session data.
    ProviderSessionDelete,
    /// Close a provider-native session without implying deletion.
    ProviderNativeSessionClose,
    /// List bounded provider-session candidates.
    ProviderSessionCatalogue,
    /// Import an exact provider-session candidate.
    ProviderSessionImport,
    /// Reconcile retained provider-session truth after interruption.
    ProviderSessionReconciliation,
    /// Read newest-first pages of provider-session history.
    ProviderSessionHistory,
    /// Reconcile retained provider-run truth after interruption.
    ProviderRunReconciliation,
    /// Clean provider resources recovered from an interrupted run.
    ProviderRecoveredResourceCleanup,
    /// Delete a bound provider-owned remote resource.
    OwnedRemoteResourceDeletion,
    /// Reattach to a retained event stream.
    StreamReattachment,
    /// Exchange realtime text, audio, or other media.
    RealtimeMedia,
    /// Roll a realtime connection before its provider limit.
    PlannedConnectionRollover,
    /// Continue direct inference after a model-requested tool call.
    DirectToolContinuation,
    /// Select or manage provider-owned inference caching.
    ProviderManagedInferenceCache,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
/// Exact set of unparameterized capabilities exposed by a driver.
pub struct CapabilityManifest {
    supported: BTreeSet<Capability>,
}

impl CapabilityManifest {
    /// Creates a deduplicated capability manifest.
    #[must_use]
    pub fn new(supported: impl IntoIterator<Item = Capability>) -> Self {
        Self {
            supported: supported.into_iter().collect(),
        }
    }

    #[must_use]
    /// Reports whether the manifest contains `capability`.
    pub fn supports(&self, capability: Capability) -> bool {
        self.supported.contains(&capability)
    }

    /// Accepts a declared capability or returns a safe rejection.
    pub fn require(&self, capability: Capability) -> Result<(), UnsupportedCapability> {
        if self.supports(capability) {
            Ok(())
        } else {
            Err(UnsupportedCapability::new(capability))
        }
    }

    /// Iterates declared capabilities in stable order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = Capability> + '_ {
        self.supported.iter().copied()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Rejection raised when a required capability is absent.
pub struct UnsupportedCapability {
    capability: Capability,
    diagnostic: SafeDiagnostic,
}

impl UnsupportedCapability {
    fn new(capability: Capability) -> Self {
        Self {
            capability,
            diagnostic: SafeDiagnostic::new(
                "swallowtail.unsupported_capability",
                format!("Adapter does not support {capability:?}"),
            ),
        }
    }

    #[must_use]
    /// Returns the capability that was required.
    pub const fn capability(&self) -> Capability {
        self.capability
    }

    #[must_use]
    /// Returns the redacted unsupported-capability diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for UnsupportedCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for UnsupportedCapability {}

#[cfg(test)]
mod tests {
    use super::{Capability, CapabilityManifest};

    #[test]
    fn unsupported_capability_fails_before_execution() {
        let manifest = CapabilityManifest::new([Capability::StructuredRun]);

        manifest
            .require(Capability::StructuredRun)
            .expect("declared capability must pass");
        let error = manifest
            .require(Capability::Resume)
            .expect_err("undeclared capability must fail");

        assert_eq!(error.capability(), Capability::Resume);
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.unsupported_capability"
        );
    }
}
