#[path = "prepared_response/instance.rs"]
mod instance;
#[path = "prepared_response/preparation.rs"]
mod preparation;
#[path = "prepared_response/profile.rs"]
mod profile;

pub use profile::{
    ClaudeCodeResponseModelSelection, ClaudeCodeResponsePreparedEvidence,
    ClaudeCodeResponsePreparedRun, ClaudeCodeResponseProfileInput,
};

use std::collections::BTreeSet;
use swallowtail_core::{
    AccessProfile, ConfiguredInstance, ConfiguredInstanceId, ExecutionHostId, HostServiceKind,
    InstalledExecutableObservation, InstanceRevision,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, PreparationFailure,
    PreparationStage, PreparedAccessEvidence, RequestId, ScopeId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs that qualify one installed Claude Code response-only instance.
pub struct ClaudeCodeResponsePreparationInput {
    pub(crate) instance_id: ConfiguredInstanceId,
    pub(crate) instance_revision: InstanceRevision,
    pub(crate) execution_host_id: ExecutionHostId,
    pub(crate) target: InstalledExecutableTarget,
    pub(crate) environment: EnvironmentRef,
    pub(crate) access_profile: AccessProfile,
    pub(crate) access_evidence: PreparedAccessEvidence,
}

impl ClaudeCodeResponsePreparationInput {
    /// Creates preparation input for an exact response-only target.
    #[must_use]
    pub const fn new(
        instance_id: ConfiguredInstanceId,
        instance_revision: InstanceRevision,
        execution_host_id: ExecutionHostId,
        target: InstalledExecutableTarget,
        environment: EnvironmentRef,
        access_profile: AccessProfile,
        access_evidence: PreparedAccessEvidence,
    ) -> Self {
        Self {
            instance_id,
            instance_revision,
            execution_host_id,
            target,
            environment,
            access_profile,
            access_evidence,
        }
    }
}

#[derive(Clone, Debug)]
/// Bounded discovery request used while preparing the response-only route.
pub struct ClaudeCodeResponsePreparationProbe {
    request_id: RequestId,
    scope_id: ScopeId,
    deadline: Deadline,
    cancellation: DiscoveryCancellation,
}

impl ClaudeCodeResponsePreparationProbe {
    /// Creates a response-only preparation probe.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        scope_id: ScopeId,
        deadline: Deadline,
        cancellation: DiscoveryCancellation,
    ) -> Self {
        Self {
            request_id,
            scope_id,
            deadline,
            cancellation,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Qualified Claude Code integration ready to prepare response-only runs.
pub struct ClaudeCodeResponsePreparedIntegration {
    environment: EnvironmentRef,
    target: InstalledExecutableTarget,
    observation: InstalledExecutableObservation,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
    instance: ConfiguredInstance,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl ClaudeCodeResponsePreparedIntegration {
    /// Returns the approved execution environment.
    #[must_use]
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    /// Returns the qualified executable target.
    #[must_use]
    pub const fn target(&self) -> &InstalledExecutableTarget {
        &self.target
    }

    /// Returns the executable observation admitted during preparation.
    #[must_use]
    pub const fn observation(&self) -> &InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the provider-owned local-subscription access profile.
    #[must_use]
    pub const fn access_profile(&self) -> &AccessProfile {
        &self.access_profile
    }

    /// Returns the prepared access evidence.
    #[must_use]
    pub const fn access_evidence(&self) -> &PreparedAccessEvidence {
        &self.access_evidence
    }

    /// Returns the exact configured provider instance.
    #[must_use]
    pub const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    /// Iterates host services present when preparation succeeded.
    pub fn available_host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.available_host_services.iter().copied()
    }

    /// Creates the low-level response-only driver bound to this integration.
    #[must_use]
    pub fn low_level_driver(&self) -> crate::ClaudeCodeResponseOnlyDriver {
        crate::ClaudeCodeResponseOnlyDriver::new(self.environment.clone())
    }

    /// Rejects host or executable drift from the prepared target.
    pub fn validate_execution_binding(
        &self,
        execution_host_id: &ExecutionHostId,
        target: &InstalledExecutableTarget,
    ) -> Result<(), PreparationFailure> {
        if execution_host_id != self.observation.execution_host_id() || target != &self.target {
            return Err(preparation::preparation_failure(
                PreparationStage::TargetSelection,
                "swallowtail.claude_code.response_only.preparation.target_drift",
                "Prepared Claude Code response-only host or executable target no longer matches",
            ));
        }
        Ok(())
    }
}

/// Discovers, validates, and prepares one Claude Code response-only instance.
pub async fn prepare_claude_code_response_only(
    input: ClaudeCodeResponsePreparationInput,
    probe: ClaudeCodeResponsePreparationProbe,
    services: HostServices,
) -> Result<ClaudeCodeResponsePreparedIntegration, PreparationFailure> {
    preparation::validate_input(&input)?;
    let available_host_services = services.available_kinds();
    let request = InstalledExecutableDiscoveryRequest::new(
        probe.request_id,
        probe.scope_id,
        input.execution_host_id.clone(),
        input.target.clone(),
        probe.deadline,
        probe.cancellation,
    );
    let driver = crate::ClaudeCodeResponseOnlyDriver::new(input.environment.clone());
    let outcome = driver
        .discover_installed_executable(request, services)
        .await
        .map_err(preparation::discovery_runtime_failure)?;
    preparation::promote(input, outcome, available_host_services)
}
